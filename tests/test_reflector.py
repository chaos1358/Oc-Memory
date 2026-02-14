"""Tests for lib/reflector.py"""

import pytest
from unittest.mock import patch
from lib.reflector import Reflector, ReflectionResult, create_reflector


class TestReflector:
    def test_init_default_model(self):
        r = Reflector(provider="openai", api_key="test")
        assert r.model == "gpt-4o-mini"

    def test_should_reflect_below_threshold(self):
        r = Reflector(api_key="test")
        assert r.should_reflect(30000, threshold=40000) is False

    def test_should_reflect_above_threshold(self):
        r = Reflector(api_key="test")
        assert r.should_reflect(50000, threshold=40000) is True

    def test_suggest_level_no_compression(self):
        r = Reflector(api_key="test")
        assert r.suggest_level(20000, target_tokens=30000) == 0

    def test_suggest_level_light(self):
        r = Reflector(api_key="test")
        assert r.suggest_level(45000, target_tokens=30000) == 1

    def test_suggest_level_medium(self):
        r = Reflector(api_key="test")
        assert r.suggest_level(90000, target_tokens=30000) == 2

    def test_suggest_level_heavy(self):
        r = Reflector(api_key="test")
        assert r.suggest_level(200000, target_tokens=30000) == 3

    def test_reflect_no_api_key(self):
        r = Reflector(api_key="")
        result = r.reflect("Some observations text", level=1)
        assert isinstance(result, ReflectionResult)
        assert result.compression_ratio == 1.0
        assert result.compressed_content == "Some observations text"

    def test_reflect_clamps_level(self):
        r = Reflector(api_key="")
        result = r.reflect("text", level=5)
        assert result.level == 3  # clamped to max

        result = r.reflect("text", level=0)
        assert result.level == 1  # clamped to min

    def test_get_stats_empty(self):
        r = Reflector(api_key="test")
        stats = r.get_stats()
        assert stats['total_compressions'] == 0
        assert stats['total_tokens_saved'] == 0

    def test_get_stats_after_reflect(self):
        r = Reflector(api_key="")
        # No API key = early return without adding to history
        r.reflect("Some text here", level=1)
        stats = r.get_stats()
        # With no API key, reflect returns without adding to history
        assert stats['total_compressions'] == 0

    def test_estimate_tokens(self):
        assert Reflector._estimate_tokens("") == 0
        assert Reflector._estimate_tokens("hello world") > 0


class TestReflectorLLMIntegration:
    """Tests for Reflector.reflect() with mocked LLM calls"""

    def test_reflect_success_level1(self):
        """Mock _call_llm and verify full reflect() pipeline at level 1"""
        r = Reflector(provider="openai", api_key="fake-key")
        original_text = (
            "- User prefers dark mode\n"
            "- User decided to use PostgreSQL\n"
            "- User prefers dark mode UI themes\n"  # duplicate-ish
            "- User wants Python 3.11 compatibility\n"
            "- Project deadline is March 2026\n"
        )
        compressed_text = (
            "- User prefers dark mode\n"
            "- Uses PostgreSQL, Python 3.11\n"
            "- Deadline: March 2026\n"
        )
        with patch.object(r, '_call_llm', return_value=compressed_text):
            result = r.reflect(original_text, level=1)

        assert isinstance(result, ReflectionResult)
        assert result.level == 1
        assert result.compression_ratio > 1.0
        assert result.compressed_tokens < result.original_tokens
        assert "PostgreSQL" in result.compressed_content
        assert len(r.history) == 1

    def test_reflect_success_level3(self):
        """Heavy compression at level 3"""
        r = Reflector(api_key="fake-key")
        original = "A " * 500  # long text
        compressed = "Summary of key points."
        with patch.object(r, '_call_llm', return_value=compressed):
            result = r.reflect(original, level=3)

        assert result.level == 3
        assert result.compression_ratio > 1.0
        assert result.compressed_content == compressed

    def test_reflect_llm_raises_exception(self):
        """LLM call fails, reflect() returns original text gracefully"""
        r = Reflector(api_key="fake-key")
        original = "Some observations"
        with patch.object(r, '_call_llm', side_effect=Exception("API error")):
            result = r.reflect(original, level=2)

        assert result.compression_ratio == 1.0
        assert result.compressed_content == original
        assert len(r.history) == 0  # failed, not added to history

    def test_reflect_stats_accumulate(self):
        """Stats accumulate correctly across multiple compressions"""
        r = Reflector(api_key="fake-key")
        with patch.object(r, '_call_llm', return_value="Short."):
            r.reflect("A longer text that has many words in it", level=1)
            r.reflect("Another long text with plenty of content", level=2)

        stats = r.get_stats()
        assert stats['total_compressions'] == 2
        assert stats['total_tokens_saved'] > 0
        assert stats['average_ratio'] > 1.0


class TestCreateReflector:
    def test_create_from_config(self):
        config = {
            'llm': {
                'provider': 'google',
                'model': 'gemini-2.5-flash',
                'api_key_env': 'GOOGLE_API_KEY',
            }
        }
        r = create_reflector(config)
        assert r.provider == "google"

    def test_create_with_defaults(self):
        r = create_reflector({})
        assert r.provider == "openai"
