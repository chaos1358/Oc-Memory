# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**OC-Memory** is an external observational memory system that adds long-term memory capabilities to [OpenClaw](https://openclaw.ai/) using a Zero-Core-Modification sidecar pattern. The system implements the [Mastra Observational Memory](https://mastra.ai/docs/memory/observational-memory) concept to provide:

- 90-day+ conversation context retention
- 90% token savings through 5-40x compression rates
- Zero modifications to OpenClaw's codebase
- Cloud backup via Obsidian + Dropbox integration
- Semantic search using ChromaDB

## Architecture

This is a **sidecar system** that operates independently from OpenClaw:

```
OpenClaw Core (unchanged)
    ↓ writes logs
    ~/.openclaw/logs/chat.log
    ↓ tail -f monitoring
OC-Memory Sidecar (external process)
    → Watcher → Observer → Merger
    → ChromaDB + active_memory.md
    → 3-Tier Memory (Hot/Warm/Cold)
```

### Key Design Principles

1. **Zero-Core-Modification**: Never modify OpenClaw's source code
2. **Sidecar Pattern**: Run as independent process, communicate via files/APIs
3. **Integration Points**:
   - Memory files: `~/.openclaw/workspace/memory/*.md` (auto-indexed by OpenClaw)
   - System Prompt injection: via `openclaw.json` or Plugin Hooks
   - Log monitoring: `~/.openclaw/logs/session-transcripts/*.jsonl`
   - HTTP API: WebSocket Gateway (port 18789) + OpenAI-compatible endpoint

## Development Commands

### Setup and Installation

```bash
# Install dependencies
pip install -r requirements.txt

# Run interactive setup wizard (recommended for first-time setup)
python setup.py

# Manual configuration (advanced users)
cp config.example.yaml config.yaml
# Edit config.yaml with your API keys and settings
```

### Development

```bash
# Run tests
pytest

# Run tests with coverage
pytest --cov=lib --cov-report=html

# Code formatting
black lib/ tests/

# Linting
flake8 lib/ tests/

# Type checking
mypy lib/
```

### Running the System

```bash
# Start memory observer daemon
python memory_observer.py

# Start file watcher (monitors user notes)
python file_watcher.py

# Monitor logs
tail -f ~/.openclaw/logs/memory_observer.log
```

## Project Structure

```
oc-memory/
├── specs/               # Requirements and design documents
│   ├── BRD.md          # Business Requirements Document
│   ├── PRD.md          # Product Requirements Document
│   ├── Tech_Spec.md    # Technical Specification
│   ├── Tasks.md        # Implementation task breakdown
│   └── CHANGES.md      # Document revision history
├── lib/                # Core library code (to be created)
│   ├── file_watcher.py # Monitors user note directories
│   ├── observer.py     # LLM-based information extraction
│   ├── memory_writer.py # Writes to OpenClaw memory system
│   └── ttl_manager.py  # Hot/Warm/Cold tier management
├── setup.py            # TUI installation wizard
├── requirements.txt    # Python dependencies
└── config.yaml         # Runtime configuration
```

## OpenClaw Integration Points

Based on analysis of the official OpenClaw repository:

### 1. Memory System (Primary Integration)

OpenClaw automatically indexes all files in `~/.openclaw/workspace/memory/`:
- Uses SQLite + sqlite-vec for vector search
- Supports full-text search (FTS5)
- Auto-updates on file changes

**Implementation**: Write processed memories as `.md` files to this directory.

### 2. System Prompt Injection

Four methods to inject memory context (in priority order):
1. **openclaw.json** - `systemPrompt.userMessage` field
2. **Plugin Hooks** - `generatePrompt` hook in openclaw.plugin.json
3. **Context Files** - Reference memory files in prompts
4. **Memory Search** - OpenClaw's built-in `/memory` command

**Implementation**: Use method #1 (openclaw.json) or #4 (memory files) for simplest integration.

### 3. Log Monitoring

Monitor OpenClaw's session transcripts:
- Location: `~/.openclaw/logs/session-transcripts/*.jsonl`
- Format: One JSON object per line with role/content/timestamp

**Implementation**: Use `tail -f` or watchdog to monitor new entries.

### 4. HTTP API (Optional)

If real-time integration is needed:
- WebSocket Gateway: Port 18789
- OpenAI-compatible endpoint: `/v1/chat/completions`

**Implementation**: POST to API for real-time memory injection (advanced feature).

## 3-Tier Memory System

### Hot Memory (0-90 days)
- **Storage**: ChromaDB + `active_memory.md`
- **Access**: Real-time semantic search
- **Size Limit**: ~10,000 observations
- **Update Frequency**: Real-time

### Warm Memory (90-365 days)
- **Storage**: Markdown archives (`~/.openclaw/workspace/memory/archive/`)
- **Access**: On-demand grep search
- **Transition**: Automatic at 90 days
- **Compression**: 5-10x via summarization

### Cold Memory (365+ days)
- **Storage**: Obsidian vault + Dropbox
- **Access**: Cloud sync, read-only
- **Transition**: Manual approval required
- **Purpose**: Long-term backup

## Implementation Roadmap

Current project status: **Planning Phase**

### Phase 1: MVP (Weeks 1-4)
- FileWatcher: Monitor user note directories
- MemoryWriter: Write to OpenClaw's memory system
- Setup Wizard: Interactive configuration
- Basic integration with OpenClaw

### Phase 2: Enhanced (Weeks 5-7)
- Observer: LLM-based information extraction (optional)
- TTL Manager: Automatic Hot→Warm→Cold transitions
- ChromaDB: Semantic search

### Phase 3: Cloud Integration (Weeks 8-9)
- Obsidian CLI integration
- Dropbox sync
- Reverse lookup (Cold→Hot)

### Phase 4: Production (Weeks 10-11)
- Performance optimization
- Comprehensive testing
- Documentation
- CI/CD setup

See [specs/Tasks.md](./specs/Tasks.md) for detailed sprint planning.

## Important Notes

### What NOT to Do

1. **Never modify OpenClaw's source code** - All integration must be external
2. **Don't create duplicate vector databases** - OpenClaw has SQLite + sqlite-vec built-in
3. **Avoid complex HTTP integrations** - File-based integration is simpler and more reliable
4. **Don't store sensitive data unencrypted** - API keys must be in config files, never committed

### Development Priorities

Current sprint (Week 1-2):
1. Set up development environment
2. Implement FileWatcher for user note directories
3. Create basic MemoryWriter for OpenClaw integration
4. Build interactive setup wizard

**Focus on simplicity**: The MVP should work with minimal dependencies and zero OpenClaw modifications.

## Testing Strategy

### Unit Tests
- Test each component in isolation
- Mock file system operations
- Mock LLM API calls

### Integration Tests
- Test with real OpenClaw instance
- Verify memory files are created correctly
- Test System Prompt injection

### Performance Tests
- Measure token savings (target: 90%+)
- Benchmark compression ratios (target: 5-10x)
- Test with 90-day conversation history

## Configuration

Key configuration parameters in `config.yaml`:

```yaml
openclaw:
  memory_dir: ~/.openclaw/workspace/memory
  log_dir: ~/.openclaw/logs/session-transcripts

watch_directories:
  - ~/Documents/notes
  - ~/Projects

hot_memory:
  ttl_days: 90
  max_observations: 10000

llm:
  provider: openai  # or google
  model: gpt-4o-mini
  api_key_env: OPENAI_API_KEY

obsidian:
  enabled: false
  vault_path: ~/Documents/ObsidianVault

dropbox:
  enabled: false
  app_key: DROPBOX_APP_KEY
```

## Resources

- **Specification Documents**: All in [specs/](./specs/) folder
- **OpenClaw Analysis**: See [openclaw_analysis_report.md](./openclaw_analysis_report.md)
- **Implementation Plan**: See [IMPLEMENTATION_ROADMAP.md](./IMPLEMENTATION_ROADMAP.md)
- **OpenClaw Repository**: https://github.com/openclaw-ai/openclaw
- **Mastra OM Concept**: https://mastra.ai/docs/memory/observational-memory
