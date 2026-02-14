"""
Reflector Agent for OC-Memory
LLM-based observation compression for long-term memory

Compresses observations to reduce token usage while
preserving key information.
"""

import json
import logging
import os
from dataclasses import dataclass
from datetime import datetime
from typing import Dict, Any, List, Optional

logger = logging.getLogger(__name__)


# =============================================================================
# Data Classes
# =============================================================================

@dataclass
class ReflectionResult:
    """Result of a reflection/compression operation"""
    original_tokens: int
    compressed_tokens: int
    compression_ratio: float
    compressed_content: str
    level: int
    timestamp: datetime


# =============================================================================
# Reflector System Prompt
# =============================================================================

REFLECTOR_SYSTEM_PROMPT = """You are a Memory Compression Agent.

Your task: Compress a list of observations into a concise summary while preserving all critical information.

## Compression Levels
- Level 1 (8/10 detail): Light compression. Keep most details, remove redundancy.
- Level 2 (6/10 detail): Medium compression. Keep key facts, summarize details.
- Level 3 (4/10 detail): Heavy compression. Keep only critical decisions and constraints.

## Rules
1. NEVER lose critical decisions or user constraints
2. Merge duplicate or similar observations
3. Preserve time references for important events
4. Group related observations together
5. Use concise language
6. Output as markdown bullet points

## Output Format
Return compressed observations as a markdown list:
- [priority_emoji] [category]: compressed observation
"""


# =============================================================================
# Reflector Agent
# =============================================================================

class Reflector:
    """
    LLM-based observation compressor.
    Reduces token usage while preserving key information.
    """

    def __init__(
        self,
        provider: str = "openai",
        model: Optional[str] = None,
        api_key: Optional[str] = None,
        api_key_env: str = "OPENAI_API_KEY",
    ):
        self.provider = provider
        self.model = model or self._default_model(provider)
        self.api_key = api_key or os.environ.get(api_key_env, "")
        self.history: List[ReflectionResult] = []

    @staticmethod
    def _default_model(provider: str) -> str:
        defaults = {
            "openai": "gpt-4o-mini",
            "google": "gemini-2.0-flash",
        }
        return defaults.get(provider, "gpt-4o-mini")

    def reflect(
        self,
        observations_text: str,
        level: int = 1,
    ) -> ReflectionResult:
        """
        Compress observations text.

        Args:
            observations_text: Raw observations markdown text
            level: Compression level (1=light, 2=medium, 3=heavy)

        Returns:
            ReflectionResult with compressed content
        """
        level = max(1, min(3, level))
        original_tokens = self._estimate_tokens(observations_text)

        if not self.api_key:
            logger.error("Cannot reflect: no API key configured")
            return ReflectionResult(
                original_tokens=original_tokens,
                compressed_tokens=original_tokens,
                compression_ratio=1.0,
                compressed_content=observations_text,
                level=level,
                timestamp=datetime.now(),
            )

        try:
            compressed = self._call_llm(observations_text, level)
            compressed_tokens = self._estimate_tokens(compressed)
            ratio = original_tokens / max(compressed_tokens, 1)

            result = ReflectionResult(
                original_tokens=original_tokens,
                compressed_tokens=compressed_tokens,
                compression_ratio=round(ratio, 1),
                compressed_content=compressed,
                level=level,
                timestamp=datetime.now(),
            )

            self.history.append(result)
            logger.info(
                f"Compression: {original_tokens} -> {compressed_tokens} tokens "
                f"({ratio:.1f}x at level {level})"
            )
            return result

        except Exception as e:
            logger.error(f"Reflection failed: {e}")
            return ReflectionResult(
                original_tokens=original_tokens,
                compressed_tokens=original_tokens,
                compression_ratio=1.0,
                compressed_content=observations_text,
                level=level,
                timestamp=datetime.now(),
            )

    def should_reflect(self, token_count: int, threshold: int = 40000) -> bool:
        """Check if compression is needed based on token count"""
        return token_count >= threshold

    def suggest_level(self, token_count: int, target_tokens: int = 30000) -> int:
        """Suggest compression level based on token count"""
        if token_count <= target_tokens:
            return 0  # No compression needed
        ratio_needed = token_count / target_tokens
        if ratio_needed < 2:
            return 1
        elif ratio_needed < 4:
            return 2
        else:
            return 3

    def get_stats(self) -> Dict[str, Any]:
        """Get compression statistics"""
        if not self.history:
            return {
                'total_compressions': 0,
                'total_tokens_saved': 0,
                'average_ratio': 0.0,
            }

        total_saved = sum(
            r.original_tokens - r.compressed_tokens for r in self.history
        )
        avg_ratio = sum(r.compression_ratio for r in self.history) / len(self.history)

        return {
            'total_compressions': len(self.history),
            'total_tokens_saved': total_saved,
            'average_ratio': round(avg_ratio, 1),
        }

    def _call_llm(self, text: str, level: int) -> str:
        """Call LLM for compression"""
        prompt = (
            f"{REFLECTOR_SYSTEM_PROMPT}\n\n"
            f"Compression Level: {level}\n\n"
            f"Compress these observations:\n\n{text}"
        )

        if self.provider == "openai":
            return self._call_openai(prompt)
        elif self.provider == "google":
            return self._call_google(prompt)
        else:
            raise ValueError(f"Unsupported provider: {self.provider}")

    def _call_openai(self, prompt: str) -> str:
        from openai import OpenAI
        client = OpenAI(api_key=self.api_key)
        response = client.chat.completions.create(
            model=self.model,
            messages=[
                {"role": "system", "content": "You are a memory compression agent."},
                {"role": "user", "content": prompt},
            ],
            temperature=0.1,
            max_tokens=4000,
        )
        return response.choices[0].message.content or ""

    def _call_google(self, prompt: str) -> str:
        from google import genai
        client = genai.Client(api_key=self.api_key)
        response = client.models.generate_content(
            model=self.model,
            contents=prompt,
        )
        return response.text

    @staticmethod
    def _estimate_tokens(text: str) -> int:
        """Estimate token count"""
        if not text:
            return 0
        return int(len(text.split()) * 1.3)


def create_reflector(config: Dict[str, Any]) -> Reflector:
    """Create a Reflector from config dictionary"""
    llm_config = config.get('llm', {})
    return Reflector(
        provider=llm_config.get('provider', 'openai'),
        model=llm_config.get('model'),
        api_key_env=llm_config.get('api_key_env', 'OPENAI_API_KEY'),
    )
