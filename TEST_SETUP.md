# Setup Wizard Test Guide

## How to Test the Setup Wizard

### Quick Test (Non-interactive)

To verify the wizard loads correctly:

```bash
python setup.py
# Press 'n' to cancel when prompted
```

### Full Interactive Test

Run the full setup wizard:

```bash
python setup.py
```

**Expected Flow:**

### Step 1: Watch Directories
- Shows 3 default options: ~/Documents/notes, ~/Projects, ~/Desktop
- Can add custom directories
- Asks about recursive watching

### Step 2: Memory Directory
- Default: ~/.openclaw/workspace/memory
- Option to customize
- Configure auto-categorization
- Set max file size (5-50 MB)

### Step 3: Logging
- Log level: INFO (recommended), DEBUG, WARNING
- Log file name: oc-memory.log
- Console output: Yes/No

### Step 4: Optional Features (Phase 2+)
- Can skip for MVP
- Advanced: LLM, ChromaDB, Obsidian
  - LLM providers: OpenAI, Google
  - API key management (.env)
  - Obsidian vault path
  - Dropbox integration

### Step 5: Review & Save
- Shows configuration summary
- Confirms before saving
- Backs up existing config.yaml
- Saves to config.yaml

### Step 6: Post-Install Instructions
- Next steps checklist
- Command examples
- Documentation links

## Test Scenarios

### Scenario 1: First-time Setup (MVP)
```bash
python setup.py
# → Ready to start? Yes
# → Watch ~/Documents/notes? Yes
# → Watch ~/Projects? No
# → Watch ~/Desktop? No
# → Add another? No
# → Recursive? Yes
# → Use default memory dir? Yes
# → Auto-categorize? Yes
# → Max file size: 10 MB
# → Log level: INFO
# → Log file: oc-memory.log
# → Console: Yes
# → Configure advanced? No
# → Save? Yes
```

**Expected Result:**
- Creates `config.yaml` with:
  ```yaml
  watch:
    dirs:
      - C:\Users\chaos\Documents\notes
    recursive: true
    poll_interval: 1.0
  memory:
    dir: C:\Users\chaos\.openclaw\workspace\memory
    auto_categorize: true
    max_file_size: 10485760
  logging:
    level: INFO
    file: oc-memory.log
    console: true
  hot_memory:
    ttl_days: 90
    max_observations: 10000
  llm:
    enabled: false
  ```

### Scenario 2: Reconfiguration
```bash
# First run
python setup.py
# ... configure and save

# Second run
python setup.py
# → Config exists, reconfigure? Yes
# → ... (goes through steps again)
```

**Expected Result:**
- Backs up existing config to `config.yaml.backup`
- Creates new config.yaml

### Scenario 3: Advanced Features
```bash
python setup.py
# → ... (basic steps)
# → Configure advanced? Yes
# → Hot memory TTL: 90
# → LLM provider: OpenAI
# → Have OPENAI_API_KEY? No
# → Enter key: sk-...
# → Enable Obsidian? Yes
# → Vault path: ~/Documents/ObsidianVault
# → Enable Dropbox? No
```

**Expected Result:**
- Creates config.yaml with LLM settings
- Creates .env file with:
  ```
  OPENAI_API_KEY=sk-...
  ```
- Sets .env permissions to 600 (Unix only)

### Scenario 4: Cancellation
```bash
python setup.py
# → Ready to start? No
```

**Expected Result:**
- "Setup cancelled." message
- No files created

### Scenario 5: Mid-flow Cancellation
```bash
python setup.py
# → ... (answer a few questions)
# → Press Ctrl+C
```

**Expected Result:**
- "Setup cancelled by user." message
- No files created (partial config not saved)

## Validation Checklist

After running setup, verify:

- [ ] `config.yaml` exists
- [ ] `config.yaml` is valid YAML
- [ ] `config.yaml` contains all required sections
- [ ] Watch directories list is correct
- [ ] Memory directory path is correct
- [ ] Logging configuration is correct
- [ ] If LLM configured: `.env` file exists
- [ ] If LLM configured: `.env` has correct permissions
- [ ] Post-install instructions are clear
- [ ] No errors in console

## Manual Verification

### Check config.yaml
```bash
cat config.yaml
```

### Validate YAML syntax
```bash
python -c "import yaml; yaml.safe_load(open('config.yaml'))"
```

### Check .env (if created)
```bash
cat .env
# Should show API keys (be careful not to expose!)
```

### Test with memory_observer.py
```bash
python memory_observer.py
# Should start without errors
# Press Ctrl+C to stop
```

## Expected Output Examples

### Successful Setup
```
╔════════════════════════════════════════════════════════════════╗
║                                                                ║
║   🧠 OC-Memory Setup Wizard                                   ║
║                                                                ║
║   External Observational Memory for OpenClaw                  ║
║   Version 0.1.0 (MVP)                                         ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝

This wizard will help you configure OC-Memory in 5 simple steps.
Setup typically takes less than 3 minutes.

? Ready to start configuration? Yes

======================================================================
📂 STEP 1: Watch Directories
======================================================================
...

✅ Configuration saved to: D:\GitHub\Oc-Memory\config.yaml

======================================================================
🎉 STEP 6: Setup Complete!
======================================================================

✅ OC-Memory is now configured!
...
```

## Troubleshooting

### questionary not found
```bash
python -m pip install questionary pyyaml python-dotenv
```

### Permission denied on .env (Unix)
```bash
chmod 600 .env
```

### Config validation errors
Check YAML syntax:
```bash
python -c "import yaml; print(yaml.safe_load(open('config.yaml')))"
```

## Next Steps After Setup

1. **Verify Configuration:**
   ```bash
   python lib/config.py
   ```

2. **Test File Watcher:**
   ```bash
   python lib/file_watcher.py
   ```

3. **Run Full System:**
   ```bash
   python memory_observer.py
   ```

4. **Create Test File:**
   ```bash
   echo "# Test" > ~/Documents/notes/test.md
   ```

5. **Check Memory Directory:**
   ```bash
   ls ~/.openclaw/workspace/memory/
   ```

---

**Test Status:** Ready for testing ✅
**Last Updated:** 2026-02-12
