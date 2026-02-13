"""
Observer Agent for OC-Memory
LLM-based observation extraction from conversation logs

Extracts structured observations from OpenClaw session transcripts
using configurable LLM providers (OpenAI, Google).
"""

import json
import logging
import os
import re
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, List, Optional

logger = logging.getLogger(__name__)


# =============================================================================
# Data Classes
# =============================================================================

@dataclass
class Observation:
    """A single extracted observation from conversation logs"""
    id: str
    timestamp: datetime
    priority: str  # 'high', 'medium', 'low'
    category: str  # 'preference', 'fact', 'task', 'decision', 'constraint'
    content: str
    metadata: Dict[str, Any] = field(default_factory=dict)

    def to_markdown(self) -> str:
        """Convert observation to markdown format"""
        priority_emoji = {
            'high': '\U0001f534',
            'medium': '\U0001f7e1',
            'low': '\U0001f7e2'
        }
        emoji = priority_emoji.get(self.priority, '\u26aa')
        ts = self.timestamp.strftime("%Y-%m-%d %H:%M")
        return f"- {emoji} [{ts}] **{self.category}**: {self.content}"

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return {
            'id': self.id,
            'timestamp': self.timestamp.isoformat(),
            'priority': self.priority,
            'category': self.category,
            'content': self.content,
            'metadata': self.metadata,
        }


# =============================================================================
# Observer System Prompt
# =============================================================================

OBSERVER_SYSTEM_PROMPT = """You are an Observation Extraction Agent for a memory system.

Your task: Analyze conversation messages and extract structured observations.

## Rules
1. Extract ONLY factual, useful information (preferences, decisions, constraints, tasks)
2. Each observation must be a single, self-contained statement
3. Assign priority: high (critical decisions/constraints), medium (useful facts), low (minor preferences)
4. Assign category: preference, fact, task, decision, constraint
5. Include time context when available
6. Do NOT include small talk, greetings, or trivial exchanges
7. Do NOT infer or assume - only extract explicitly stated information

## Output Format
Return a JSON array of observations:
```json
[
  {
    "priority": "high|medium|low",
    "category": "preference|fact|task|decision|constraint",
    "content": "Clear, concise observation statement",
    "time_context": "optional time reference from the conversation"
  }
]
```

If no meaningful observations can be extracted, return an empty array: []
"""


# =============================================================================
# Observer Agent
# =============================================================================

class Observer:
    """
    LLM-based observation extraction agent.
    Analyzes conversation logs and extracts structured observations.
    """

    def __init__(
        self,
        provider: str = "openai",
        model: Optional[str] = None,
        api_key: Optional[str] = None,
        api_key_env: str = "OPENAI_API_KEY",
    ):
        """
        Args:
            provider: LLM provider ('openai' or 'google')
            model: Model name (auto-selected if None)
            api_key: API key (reads from env if None)
            api_key_env: Environment variable name for API key
        """
        self.provider = provider
        self.model = model or self._default_model(provider)
        self.api_key = api_key or os.environ.get(api_key_env, "")
        self._observation_counter = 0

        if not self.api_key:
            logger.warning(
                f"No API key found. Set {api_key_env} environment variable "
                f"or pass api_key parameter."
            )

    @staticmethod
    def _default_model(provider: str) -> str:
        """Get default model for provider"""
        defaults = {
            "openai": "gpt-4o-mini",
            "google": "gemini-2.5-flash",
        }
        return defaults.get(provider, "gpt-4o-mini")

    def observe(self, messages: List[Dict[str, str]]) -> List[Observation]:
        """
        Extract observations from conversation messages.

        Args:
            messages: List of message dicts with 'role' and 'content' keys

        Returns:
            List of extracted Observation objects
        """
        if not messages:
            return []

        if not self.api_key:
            logger.error("Cannot observe: no API key configured")
            return []

        # Format messages for the LLM
        conversation_text = self._format_messages(messages)

        # Call LLM
        try:
            raw_response = self._call_llm(conversation_text)
            observations = self._parse_response(raw_response)
            logger.info(f"Extracted {len(observations)} observations")
            return observations
        except Exception as e:
            logger.error(f"Observation extraction failed: {e}")
            return []

    def observe_from_file(self, log_file: Path) -> List[Observation]:
        """
        Extract observations from a JSONL log file.

        Args:
            log_file: Path to .jsonl session transcript

        Returns:
            List of extracted Observation objects
        """
        messages = self._read_jsonl_log(log_file)
        return self.observe(messages)

    def _format_messages(self, messages: List[Dict[str, str]]) -> str:
        """Format messages into a conversation string"""
        lines = []
        for msg in messages:
            role = msg.get('role', 'unknown').capitalize()
            content = msg.get('content', '')
            lines.append(f"[{role}]: {content}")
        return "\n\n".join(lines)

    def _call_llm(self, conversation_text: str) -> str:
        """
        Call the LLM API to extract observations.

        Args:
            conversation_text: Formatted conversation text

        Returns:
            Raw LLM response string
        """
        if self.provider == "openai":
            return self._call_openai(conversation_text)
        elif self.provider == "google":
            return self._call_google(conversation_text)
        else:
            raise ValueError(f"Unsupported provider: {self.provider}")

    def _call_openai(self, conversation_text: str) -> str:
        """Call OpenAI API"""
        from openai import OpenAI

        client = OpenAI(api_key=self.api_key)
        response = client.chat.completions.create(
            model=self.model,
            messages=[
                {"role": "system", "content": OBSERVER_SYSTEM_PROMPT},
                {"role": "user", "content": f"Extract observations from this conversation:\n\n{conversation_text}"},
            ],
            temperature=0.1,
            max_tokens=2000,
            response_format={"type": "json_object"},
        )
        return response.choices[0].message.content or "[]"

    def _call_google(self, conversation_text: str) -> str:
        """Call Google Gemini API"""
        import google.generativeai as genai

        genai.configure(api_key=self.api_key)
        model = genai.GenerativeModel(self.model)

        prompt = (
            f"{OBSERVER_SYSTEM_PROMPT}\n\n"
            f"Extract observations from this conversation:\n\n"
            f"{conversation_text}\n\n"
            f"Return ONLY a JSON array."
        )

        response = model.generate_content(prompt)
        return response.text

    def _parse_response(self, raw_response: str) -> List[Observation]:
        """
        Parse LLM response into Observation objects.

        Args:
            raw_response: Raw LLM response string

        Returns:
            List of Observation objects
        """
        # Try to extract JSON from response
        try:
            data = json.loads(raw_response)
        except json.JSONDecodeError:
            # Try to find JSON array in the response
            match = re.search(r'\[.*\]', raw_response, re.DOTALL)
            if match:
                try:
                    data = json.loads(match.group())
                except json.JSONDecodeError:
                    logger.warning("Failed to parse LLM response as JSON")
                    return []
            else:
                logger.warning("No JSON array found in LLM response")
                return []

        # Handle wrapper object (e.g., {"observations": [...]})
        if isinstance(data, dict):
            for key in ('observations', 'results', 'data', 'items'):
                if key in data and isinstance(data[key], list):
                    data = data[key]
                    break
            else:
                logger.warning("Unexpected JSON structure")
                return []

        if not isinstance(data, list):
            return []

        # Convert to Observation objects
        observations = []
        now = datetime.now()

        for item in data:
            if not isinstance(item, dict):
                continue

            self._observation_counter += 1
            obs_id = f"obs_{now.strftime('%Y%m%d')}_{self._observation_counter:04d}"

            # Validate priority
            priority = item.get('priority', 'medium').lower()
            if priority not in ('high', 'medium', 'low'):
                priority = 'medium'

            # Validate category
            category = item.get('category', 'fact').lower()
            valid_categories = ('preference', 'fact', 'task', 'decision', 'constraint')
            if category not in valid_categories:
                category = 'fact'

            content = item.get('content', '').strip()
            if not content:
                continue

            obs = Observation(
                id=obs_id,
                timestamp=now,
                priority=priority,
                category=category,
                content=content,
                metadata={
                    'time_context': item.get('time_context', ''),
                    'source': 'observer',
                },
            )
            observations.append(obs)

        return observations

    @staticmethod
    def _read_jsonl_log(log_file: Path) -> List[Dict[str, str]]:
        """
        Read messages from a JSONL log file.

        Args:
            log_file: Path to .jsonl file

        Returns:
            List of message dicts
        """
        messages = []
        try:
            with open(log_file, 'r', encoding='utf-8') as f:
                for line in f:
                    line = line.strip()
                    if not line:
                        continue
                    try:
                        entry = json.loads(line)
                        if 'role' in entry and 'content' in entry:
                            messages.append({
                                'role': entry['role'],
                                'content': entry['content'],
                            })
                    except json.JSONDecodeError:
                        continue
        except FileNotFoundError:
            logger.error(f"Log file not found: {log_file}")
        except Exception as e:
            logger.error(f"Error reading log file: {e}")

        return messages


# =============================================================================
# Convenience functions
# =============================================================================

def create_observer(config: Dict[str, Any]) -> Observer:
    """
    Create an Observer from config dictionary.

    Args:
        config: Configuration dict with 'llm' section

    Returns:
        Configured Observer instance
    """
    llm_config = config.get('llm', {})
    provider = llm_config.get('provider', 'openai')
    model = llm_config.get('model')
    api_key_env = llm_config.get('api_key_env', 'OPENAI_API_KEY')

    return Observer(
        provider=provider,
        model=model,
        api_key_env=api_key_env,
    )
