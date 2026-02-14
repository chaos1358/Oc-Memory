"""Tests for lib/observer.py"""

import json
import pytest
from datetime import datetime
from pathlib import Path
from unittest.mock import patch, MagicMock

from lib.observer import Observer, Observation, OBSERVER_SYSTEM_PROMPT, create_observer


class TestObservation:
    def test_to_markdown(self):
        obs = Observation(
            id="obs_001",
            timestamp=datetime(2026, 2, 13, 10, 30),
            priority="high",
            category="decision",
            content="Use ChromaDB for vector storage",
        )
        md = obs.to_markdown()
        assert "2026-02-13 10:30" in md
        assert "decision" in md
        assert "ChromaDB" in md

    def test_to_markdown_priority_emoji(self):
        for priority, expected_char in [("high", "\U0001f534"), ("medium", "\U0001f7e1"), ("low", "\U0001f7e2")]:
            obs = Observation(
                id="test", timestamp=datetime.now(),
                priority=priority, category="fact", content="test",
            )
            assert expected_char in obs.to_markdown()

    def test_to_dict(self):
        obs = Observation(
            id="obs_001",
            timestamp=datetime(2026, 1, 1),
            priority="medium",
            category="fact",
            content="Test fact",
            metadata={"source": "test"},
        )
        d = obs.to_dict()
        assert d['id'] == "obs_001"
        assert d['priority'] == "medium"
        assert d['metadata']['source'] == "test"


class TestObserver:
    def test_init_default_model(self):
        obs = Observer(provider="openai", api_key="test")
        assert obs.model == "gpt-4o-mini"

    def test_init_google_model(self):
        obs = Observer(provider="google", api_key="test")
        assert obs.model == "gemini-2.5-flash"

    def test_observe_empty_messages(self):
        obs = Observer(api_key="test")
        result = obs.observe([])
        assert result == []

    def test_observe_no_api_key(self):
        obs = Observer(api_key="")
        result = obs.observe([{"role": "user", "content": "test"}])
        assert result == []

    def test_format_messages(self):
        obs = Observer(api_key="test")
        messages = [
            {"role": "user", "content": "Hello"},
            {"role": "assistant", "content": "Hi there"},
        ]
        formatted = obs._format_messages(messages)
        assert "[User]: Hello" in formatted
        assert "[Assistant]: Hi there" in formatted

    def test_parse_response_valid_json_array(self):
        obs = Observer(api_key="test")
        response = json.dumps([
            {"priority": "high", "category": "decision", "content": "Use Python"},
            {"priority": "low", "category": "preference", "content": "Dark mode"},
        ])
        results = obs._parse_response(response)
        assert len(results) == 2
        assert results[0].priority == "high"
        assert results[0].category == "decision"
        assert results[1].content == "Dark mode"

    def test_parse_response_wrapper_object(self):
        obs = Observer(api_key="test")
        response = json.dumps({
            "observations": [
                {"priority": "medium", "category": "fact", "content": "Test"}
            ]
        })
        results = obs._parse_response(response)
        assert len(results) == 1

    def test_parse_response_invalid_json(self):
        obs = Observer(api_key="test")
        results = obs._parse_response("not json at all")
        assert results == []

    def test_parse_response_embedded_json(self):
        obs = Observer(api_key="test")
        response = 'Here are the observations:\n[{"priority": "high", "category": "fact", "content": "Test"}]\nDone.'
        results = obs._parse_response(response)
        assert len(results) == 1

    def test_parse_response_empty_content_skipped(self):
        obs = Observer(api_key="test")
        response = json.dumps([
            {"priority": "high", "category": "fact", "content": ""},
            {"priority": "high", "category": "fact", "content": "Valid"},
        ])
        results = obs._parse_response(response)
        assert len(results) == 1
        assert results[0].content == "Valid"

    def test_parse_response_invalid_priority_normalized(self):
        obs = Observer(api_key="test")
        response = json.dumps([
            {"priority": "URGENT", "category": "fact", "content": "Test"},
        ])
        results = obs._parse_response(response)
        assert results[0].priority == "medium"  # normalized to default

    def test_parse_response_invalid_category_normalized(self):
        obs = Observer(api_key="test")
        response = json.dumps([
            {"priority": "high", "category": "unknown_cat", "content": "Test"},
        ])
        results = obs._parse_response(response)
        assert results[0].category == "fact"  # normalized to default

    def test_read_jsonl_log(self, temp_dir):
        log_file = temp_dir / "test.jsonl"
        lines = [
            json.dumps({"role": "user", "content": "Hello"}),
            json.dumps({"role": "assistant", "content": "Hi"}),
            "invalid json line",
            json.dumps({"no_role": True}),  # missing role
        ]
        log_file.write_text('\n'.join(lines))

        messages = Observer._read_jsonl_log(log_file)
        assert len(messages) == 2
        assert messages[0]['role'] == "user"

    def test_read_jsonl_log_missing_file(self, temp_dir):
        messages = Observer._read_jsonl_log(temp_dir / "missing.jsonl")
        assert messages == []


class TestObserverLLMIntegration:
    """Tests for Observer.observe() with mocked LLM calls"""

    def test_observe_success_openai(self):
        """Mock OpenAI _call_llm and verify full observe() pipeline"""
        obs = Observer(provider="openai", api_key="fake-key")
        mock_response = json.dumps([
            {"priority": "high", "category": "decision", "content": "Use PostgreSQL for storage"},
            {"priority": "medium", "category": "preference", "content": "Prefer dark mode UI"},
        ])
        with patch.object(obs, '_call_llm', return_value=mock_response):
            messages = [
                {"role": "user", "content": "I decided to use PostgreSQL for storage"},
                {"role": "assistant", "content": "Good choice. Do you prefer dark or light mode?"},
                {"role": "user", "content": "Dark mode please"},
            ]
            result = obs.observe(messages)

        assert len(result) == 2
        assert result[0].priority == "high"
        assert result[0].category == "decision"
        assert "PostgreSQL" in result[0].content
        assert result[1].priority == "medium"
        assert isinstance(result[0], Observation)

    def test_observe_success_google(self):
        """Mock Google _call_llm and verify observe() works with google provider"""
        obs = Observer(provider="google", api_key="fake-key")
        mock_response = json.dumps([
            {"priority": "low", "category": "fact", "content": "Project uses Python 3.11"},
        ])
        with patch.object(obs, '_call_llm', return_value=mock_response):
            result = obs.observe([{"role": "user", "content": "We use Python 3.11"}])

        assert len(result) == 1
        assert result[0].category == "fact"
        assert "Python 3.11" in result[0].content

    def test_observe_llm_returns_empty_array(self):
        """LLM returns valid JSON but no observations"""
        obs = Observer(api_key="fake-key")
        with patch.object(obs, '_call_llm', return_value="[]"):
            result = obs.observe([{"role": "user", "content": "Hello"}])

        assert result == []

    def test_observe_llm_raises_exception(self):
        """LLM call fails, observe() returns empty list gracefully"""
        obs = Observer(api_key="fake-key")
        with patch.object(obs, '_call_llm', side_effect=Exception("API timeout")):
            result = obs.observe([{"role": "user", "content": "test"}])

        assert result == []

    def test_observe_llm_returns_malformed_json(self):
        """LLM returns non-JSON response, handled gracefully"""
        obs = Observer(api_key="fake-key")
        with patch.object(obs, '_call_llm', return_value="I found no observations."):
            result = obs.observe([{"role": "user", "content": "test"}])

        assert result == []


class TestCreateObserver:
    def test_create_from_config(self):
        config = {
            'llm': {
                'provider': 'google',
                'model': 'gemini-2.5-flash',
                'api_key_env': 'GOOGLE_API_KEY',
            }
        }
        obs = create_observer(config)
        assert obs.provider == "google"
        assert obs.model == "gemini-2.5-flash"

    def test_create_with_defaults(self):
        obs = create_observer({})
        assert obs.provider == "openai"
        assert obs.model == "gpt-4o-mini"
