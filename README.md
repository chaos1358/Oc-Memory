# OC-Memory

**OpenClaw never forgets.**

[![Python 3.8+](https://img.shields.io/badge/python-3.8+-3776AB?logo=python&logoColor=white)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/guardian-Rust-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tests: 228](https://img.shields.io/badge/tests-228_passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)]()

---

# 💾 OC-Memory & 🛡️ OC-Guardian

OC-Memory는 OpenClaw 가 90일 전의 대화도 생생하게 기억하며, 지연 없이 빠르게 답변을 제공하는 지능형 메모리 시스템입니다.

---

## 🧠 OC-Memory: AI의 똑똑한 기억 저장소
AI가 예전에 했던 말을 잊지 않도록 도와주는 시스템입니다.

| 기능 | 기술 명칭 (Technical) | 눈높이 설명 (Easy) |
| :--- | :--- | :--- |
| **체계적 저장** | 3-Tier 메모리 아키텍처 | 기억을 3단계로 나누어 중요한 것부터 차곡차곡 정리해요. |
| **비용 절감** | AI 메모리 압축 (Mastra) | 기억을 작게 접어서 보관해 AI 사용료를 90%나 아껴요. |
| **간편 설치** | Zero-Core-Modification | 원래 프로그램을 고치지 않고도 새로운 능력을 쏙 추가해요. |
| **자동 청소** | 데이터 수명 주기 관리 (TTL) | 너무 오래되어 필요 없는 기억은 스스로 지워 용량을 지켜요. |
| **찰떡 이해** | 의미 기반 검색 (Semantic Search) | 단어가 조금 틀려도 "그거 있잖아!" 하면 뜻을 이해하고 찾아내요. |

---

## 🛡️ OC-Guardian: 든든한 AI 관리자 (NEW!!)
OC-Guardian은 **Rust 기반**으로 제작되어, OpenClaw와 OC-Memory가 안정적으로 돌아가도록 감시하고 지켜주는 '보디가드' 시스템입니다.

> **"OpenClaw가 멈추거나 서버가 다운되는 것을 절대 허용하지 않습니다."**

### ✨ 주요 기능
* **⚡ 원터치 실행**: `oc-guardian up` 한 줄이면 OpenClaw → OC-Memory 순서로 자동 실행됩니다.
* **🩺 5단계 정밀 검진**: 프로그램이 아픈 곳은 없는지 5단계로 나누어 꼼꼼하게 체크해요.
* **🚑 자동 응급처치**: 문제가 생기면 정해진 시나리오에 따라 스스로 복구하고 다시 시작해요.
* **🧹 무한 용량 관리**: 기록(로그)이 쌓여서 컴퓨터가 느려지지 않게 자동으로 청소해요.
* **📧 즉시 알림**: 터미널이 재시작되는 등 중요한 일이 생기면 이메일로 바로 알려줘요.

---

### 🚀 시작하기

```bash
# 전체 스택 시작 (OpenClaw → OC-Memory 순서로 자동 실행)
oc-guardian up

# 전체 스택 종료 (OC-Memory → OpenClaw 역순 종료)
oc-guardian down

# Guardian 감시 프로세스만 시작 (OpenClaw이 이미 실행 중일 때)
oc-guardian start

# Guardian 감시 프로세스만 종료
oc-guardian stop

# 특정 프로세스 또는 전체 재시작
oc-guardian restart [process-name]

# 실시간 상태 확인
oc-guardian status

# 로그 확인
oc-guardian logs [process-name] --follow --tail 50
```

| What | How |
|------|-----|
| **90% 토큰 절약** | 5-40x 압축으로 비용 대폭 절감 |
| **Zero-Core-Mod** | OpenClaw 소스 수정 없이 사이드카로 동작 |
| **3-Tier Memory** | Hot → Warm → Cold 자동 전이 |
| **3MB Guardian** | Rust 바이너리가 프로세스 관리 + 5초 내 자동 복구 |
| **Cloud Backup** | Obsidian vault + Dropbox 자동 동기화 |

---

## 🌐 English Version

### 💾 OC-Memory & 🛡️ OC-Guardian

OC-Memory is an intelligent memory system that enables OpenClaw to vividly remember conversations from 90 days ago and provide fast responses without delay.

---

### 🧠 OC-Memory: Smart Memory Storage for AI
A system that helps AI remember what it said before and never forget.

| Feature | Technical Term | Easy Explanation |
| :--- | :--- | :--- |
| **Organized Storage** | 3-Tier Memory Architecture | Organizes memories into 3 levels, keeping important ones first. |
| **Cost Reduction** | AI Memory Compression (Mastra) | Folds memories to save up to 90% on AI usage costs. |
| **Easy Installation** | Zero-Core-Modification | Adds new capabilities without modifying the original program. |
| **Auto Cleanup** | Data Lifecycle Management (TTL) | Automatically removes old, unnecessary memories to save space. |
| **Smart Understanding** | Semantic Search | Finds what you mean even if the words are slightly different. |

---

### 🛡️ OC-Guardian: Your Reliable AI Manager (NEW!!)
OC-Guardian is built with **Rust** and acts as a 'bodyguard' system that monitors and protects OpenClaw and OC-Memory to ensure stable operation.

> **"Never allows OpenClaw to stop or the server to go down."**

### ✨ Key Features
* **⚡ One-Touch Execution**: Just run `oc-guardian up` — it auto-starts OpenClaw → OC-Memory in the correct order.
* **🩺 5-Level Health Check**: Thoroughly checks the program's health across 5 levels.
* **🚑 Auto Recovery**: Self-recovers and restarts according to predefined scenarios when issues occur.
* **🧹 Unlimited Capacity Management**: Automatically cleans up logs to prevent computer slowdown.
* **📧 Instant Notifications**: Sends email alerts immediately when critical events like terminal restarts occur.

---

### 🚀 Getting Started

```bash
# Start entire stack (OpenClaw → OC-Memory, auto-ordered)
oc-guardian up

# Shut down entire stack (OC-Memory → OpenClaw, reverse order)
oc-guardian down

# Start guardian supervisor only (when OpenClaw is already running)
oc-guardian start

# Stop guardian supervisor only
oc-guardian stop

# Restart specific process or all
oc-guardian restart [process-name]

# Live status table
oc-guardian status

# View logs
oc-guardian logs [process-name] --follow --tail 50
```

| What | How |
|------|-----|
| **90% Token Savings** | 5-40x compression for massive cost reduction |
| **Zero-Core-Mod** | Works as sidecar without modifying OpenClaw source |
| **3-Tier Memory** | Automatic Hot → Warm → Cold transition |
| **3MB Guardian** | Rust binary handles process management + auto-recovery in 5 seconds |
| **Cloud Backup** | Auto-sync with Obsidian vault + Dropbox |

---

## Architecture

```
OpenClaw (unchanged)
    │ writes logs
    ▼
~/.openclaw/logs/session-transcripts/*.jsonl
    │ tail -f monitoring
    ▼
┌─────────────────────────────────────────────┐
│              OC-Memory Sidecar              │
│                                             │
│  Watcher → Observer → Merger → Reflector    │
│                    │                        │
│         ┌─────────┼─────────┐              │
│         ▼         ▼         ▼              │
│   ┌──────────┬──────────┬──────────┐       │
│   │   Hot    │   Warm   │   Cold   │       │
│   │ 0-90d   │ 90-365d  │  365d+   │       │
│   │ ChromaDB│ Archives │ Obsidian │       │
│   │ realtime│ on-demand│ Dropbox  │       │
│   └──────────┴──────────┴──────────┘       │
│                    │                        │
│         Unified Search Engine               │
└─────────────────────────────────────────────┘
    │ writes memory files
    ▼
~/.openclaw/workspace/memory/*.md
    │ auto-indexed
    ▼
OpenClaw memory_search (SQLite + sqlite-vec)
```

---

## Quick Start

### 1. Install

```bash
git clone https://github.com/[username]/oc-memory.git
cd oc-memory
pip install -r requirements.txt
```

### 2. Configure

```bash
# Interactive setup wizard (recommended)
python setup.py

# Or manual
cp config.example.yaml config.yaml
```

### 3. Run

```bash
python memory_observer.py
```

### 4. Verify

Create a test note in your watch directory:

```bash
echo "# Test Memory" > ~/Documents/notes/test.md
```

The observer detects it, processes it, and writes to `~/.openclaw/workspace/memory/`. OpenClaw auto-indexes within seconds.

---

## OC-Guardian

Rust 기반 프로세스 가디언. OpenClaw 상태를 감시하고, OC-Memory를 자동으로 시작, 모니터링, 복구합니다.

```bash
# Build (~3MB binary)
cd guardian
cargo build --release

# Setup configuration
cp guardian.toml.example guardian.toml
# Edit guardian.toml to define your processes
```

**`oc-guardian status` 출력 예시**:

```
┌───────────┬────────┬──────┬──────────┬─────────┬───────────┐
│ Name      │ Status │ PID  │ Uptime   │ Restarts│ Health    │
├───────────┼────────┼──────┼──────────┼─────────┼───────────┤
│ openclaw  │ online │ 1234 │ 10m 5s   │ 0       │ ✓ Healthy │
│ oc-memory │ online │ 5678 │ 9m 55s   │ 0       │ ✓ Healthy │
└───────────┴────────┴──────┴──────────┴─────────┴───────────┘
```

### Features

| Feature | Detail |
|---------|--------|
| **5-Level Health Check** | Process alive → Log activity → Config validation → Resource monitoring → HTTP endpoint |
| **6 Recovery Scenarios** | invalid_json, process_crash, unresponsive, memory_leak, network_timeout, llm_error |
| **Dependency-based Start** | Topological sort ensures correct startup order |
| **Log Rotation** | Size-based with generational backups |
| **Memory Compression** | Dual-trigger: token threshold (30K) + 24h schedule |
| **Email Alerts** | SMTP notifications for crashes and recovery events |

### Run in Background

**Option 1: macOS LaunchAgent (recommended)**

```bash
# Install service
cd guardian/service && ./install-service.sh install

# Start via launchd
launchctl kickstart -k gui/$(id -u)/com.openclaw.guardian

# Stop
oc-guardian stop

# Guardian auto-restarts on crash (ThrottleInterval: 30s)
# Clean stop (exit 0) does NOT restart — use oc-guardian stop
```

**Option 2: Linux systemd**

```bash
cd guardian/service && sudo ./install-service.sh install
sudo systemctl start oc-guardian
```

**Option 3: nohup/screen (quick)**

```bash
nohup ./target/release/oc-guardian up &
```

**Note**: Guardian monitors OpenClaw as an external process (`managed = false`) — it does not spawn or stop OpenClaw directly. Use `oc-guardian up/down` to orchestrate the full stack.

See [guardian/README.md](guardian/README.md) for full documentation.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Memory Engine** | Python 3.8+, ChromaDB, watchdog |
| **Process Guardian** | Rust 2021, tokio, clap, sysinfo |
| **LLM** | OpenAI GPT-4o-mini, Google Gemini |
| **Cloud** | Obsidian vault, Dropbox API |
| **Service** | systemd, launchd, Windows Scheduled Task |
| **CI/CD** | GitHub Actions (Windows + macOS + Linux) |

---

## Project Structure

```
oc-memory/
├── lib/                  # Python core (15 modules)
│   ├── config.py         # YAML configuration
│   ├── file_watcher.py   # Directory monitoring
│   ├── memory_writer.py  # OpenClaw memory integration
│   ├── observer.py       # LLM-based extraction
│   ├── memory_merger.py  # Deduplication & merge
│   ├── reflector.py      # Periodic summarization
│   ├── memory_store.py   # ChromaDB vector store
│   ├── ttl_manager.py    # Hot/Warm/Cold transitions
│   ├── unified_search.py # 3-tier search engine
│   ├── obsidian_client.py# Obsidian vault CRUD
│   ├── dropbox_sync.py   # Dropbox sync
│   ├── error_handler.py  # Retry & error handling
│   ├── api_detector.py   # LLM API auto-detection
│   └── error_notifier.py # Error notifications
├── guardian/              # Rust process guardian
│   ├── src/              # 11 Rust modules
│   ├── service/          # systemd, launchd, Windows templates
│   └── README.md         # Guardian docs
├── tests/                # 164 Python tests
├── specs/                # BRD, PRD, Tech Spec, Tasks
├── scripts/              # Build & install scripts
├── memory_observer.py    # Main daemon
├── setup.py              # TUI setup wizard
└── config.example.yaml   # Example configuration
```

---

## Testing

```bash
# Python tests (164 tests)
pytest

# Guardian tests (64 tests)
cd guardian && cargo test
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [specs/BRD.md](specs/BRD.md) | Business Requirements |
| [specs/PRD.md](specs/PRD.md) | Product Requirements |
| [specs/Tech_Spec.md](specs/Tech_Spec.md) | Technical Specification |
| [specs/Tasks.md](specs/Tasks.md) | Implementation Tasks (4 Phase, 11 weeks) |
| [guardian/README.md](guardian/README.md) | Guardian detailed documentation |
| [Patch_note/RELEASE_v1.0.md](Patch_note/RELEASE_v1.0.md) | v1.0 Release Notes |
| [CHANGELOG.md](CHANGELOG.md) | v0.2.0 Changelog |

---

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

MIT License - see [LICENSE](LICENSE)

---

## Acknowledgments

- [Mastra](https://mastra.ai/) — Observational Memory concept
- [OpenClaw](https://openclaw.ai/) — Open-source AI agent framework
- [Obsidian](https://obsidian.md/) — Knowledge management
- [ChromaDB](https://www.trychroma.com/) — AI-native vector database
