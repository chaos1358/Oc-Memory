# OC-Memory Quick Start Guide

## ✅ Installation Complete!

All core components have been successfully implemented and tested:

- ✅ FileWatcher - Monitors directories for .md files
- ✅ MemoryWriter - Copies files to OpenClaw memory
- ✅ Config System - YAML-based configuration
- ✅ Main Daemon - Integrated observer process

## 🚀 How to Run

### 1. Configuration

The default config file `config.yaml` is already set up:

```yaml
watch:
  dirs:
    - ~/Documents/notes
    - ~/Projects

memory:
  dir: ~/.openclaw/workspace/memory
```

**Customize if needed:**
```bash
notepad config.yaml
```

### 2. Start the Observer

```bash
python memory_observer.py
```

You should see:
```
========================================
Starting OC-Memory Observer
========================================
Watch directories: ['C:\\Users\\chaos\\Documents\\notes', ...]
Memory directory: C:\Users\chaos\.openclaw\workspace\memory
========================================
OC-Memory Observer started successfully
Monitoring for file changes... (Press Ctrl+C to stop)
```

### 3. Test It!

**In another terminal or folder:**

```bash
# Create a test note
echo "# My First Memory" > ~/Documents/notes/test_note.md
echo "" >> ~/Documents/notes/test_note.md
echo "This is a test memory entry for OC-Memory." >> ~/Documents/notes/test_note.md
```

**Expected behavior:**
1. FileWatcher detects the new file
2. MemoryWriter copies it to `~/.openclaw/workspace/memory/notes/`
3. Metadata is added (timestamp, category, source path)
4. OpenClaw auto-indexes it within ~5 seconds

### 4. Verify in OpenClaw

**Check the memory directory:**
```bash
ls ~/.openclaw/workspace/memory/notes/
```

**In OpenClaw CLI:**
```
Use memory_search tool to find "test memory"
```

## 🧪 Testing Components Individually

### Test FileWatcher
```bash
python lib/file_watcher.py
# In another terminal: create/edit .md files in ~/Documents/notes
```

### Test MemoryWriter
```bash
python lib/memory_writer.py
# Runs built-in tests automatically
```

### Test Config
```bash
python lib/config.py
# Shows loaded configuration
```

## 📁 Directory Structure

```
~/.openclaw/workspace/memory/
├── notes/           # Auto-categorized notes
│   └── test_note.md
├── projects/        # Project-related files
├── documents/       # General documents
└── general/         # Uncategorized files
```

## 🔧 Configuration Options

### Watch Directories

Add or modify directories to monitor:

```yaml
watch:
  dirs:
    - ~/Documents/notes
    - ~/Projects
    - ~/Desktop/scratch
```

### Memory Directory

**Important:** Match this to OpenClaw's memory path:

```yaml
memory:
  dir: ~/.openclaw/workspace/memory
```

### Logging

```yaml
logging:
  level: INFO  # DEBUG for verbose output
  file: oc-memory.log
  console: true
```

## 🐛 Troubleshooting

### Files Not Being Detected

1. **Check watch directories exist:**
   ```bash
   ls ~/Documents/notes
   ```

2. **Enable debug logging:**
   Edit `config.yaml`:
   ```yaml
   logging:
     level: DEBUG
   ```

3. **Check logs:**
   ```bash
   tail -f oc-memory.log
   ```

### Memory Files Not Created

1. **Check memory directory permissions:**
   ```bash
   ls -la ~/.openclaw/workspace/
   ```

2. **Verify config path:**
   ```bash
   python lib/config.py
   ```

### OpenClaw Not Finding Memories

1. **Wait 5-10 seconds** for auto-indexing

2. **Check OpenClaw memory database:**
   ```bash
   ls ~/.openclaw/agents/main/memory.db
   ```

3. **Verify file format** (must be valid Markdown)

## 📊 Next Steps

### Phase 1 (Current - MVP)
- ✅ FileWatcher implementation
- ✅ MemoryWriter implementation
- ✅ Basic configuration
- ⏳ Setup wizard (TUI)
- ⏳ OpenClaw integration testing

### Phase 2 (Weeks 3-4)
- LLM-based observation extraction
- Webhook integration
- TTL management (Hot/Warm/Cold tiers)

### Phase 3 (Weeks 5-7)
- ChromaDB semantic search
- Obsidian integration
- Dropbox sync

## 📚 Documentation

- **[BRD.md](specs/BRD.md)** - Business requirements
- **[PRD.md](specs/PRD.md)** - Product requirements
- **[Tech_Spec.md](specs/Tech_Spec.md)** - Technical specification
- **[Tasks.md](specs/Tasks.md)** - Implementation roadmap
- **[IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)** - Detailed week-by-week plan

## 💡 Tips

1. **Use consistent naming** in your notes for better categorization
2. **Add frontmatter** to your markdown files for better metadata
3. **Monitor the logs** during initial testing to understand behavior
4. **Start with one directory** and expand after confirming it works

## 🎯 Success Criteria

You'll know it's working when:

1. ✅ Observer starts without errors
2. ✅ Creating/editing .md files triggers log messages
3. ✅ Files appear in `~/.openclaw/workspace/memory/`
4. ✅ OpenClaw's memory_search finds your content

## 🚀 Running in Production

### As a Background Service (Linux/macOS)

**systemd (Linux):**
```bash
# Create service file
sudo nano /etc/systemd/system/oc-memory.service

# Enable and start
sudo systemctl enable oc-memory
sudo systemctl start oc-memory
```

**LaunchAgent (macOS):**
```bash
# Create plist file
nano ~/Library/LaunchAgents/com.oc-memory.observer.plist

# Load
launchctl load ~/Library/LaunchAgents/com.oc-memory.observer.plist
```

### Windows Service

Use NSSM (Non-Sucking Service Manager) or Task Scheduler.

---

**Version:** 0.1.0 (MVP)
**Status:** Core Implementation Complete ✅
**Last Updated:** 2026-02-12
