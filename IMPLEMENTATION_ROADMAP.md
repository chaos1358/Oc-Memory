# OC-Memory 구현 로드맵 (Implementation Roadmap)

**프로젝트**: OC-Memory - OpenClaw Observational Memory System
**버전**: 1.0
**작성일**: 2026-02-12
**상태**: Ready to Start

---

## 목차

1. [Quick Start](#1-quick-start)
2. [현재 상태 및 우선순위](#2-현재-상태-및-우선순위)
3. [Sprint 1: 즉시 시작 가능한 작업](#3-sprint-1-즉시-시작-가능한-작업)
4. [개발 환경 설정](#4-개발-환경-설정)
5. [Week 1 작업 계획](#5-week-1-작업-계획)
6. [Week 2 작업 계획](#6-week-2-작업-계획)
7. [체크포인트 및 검증](#7-체크포인트-및-검증)

---

## 1. Quick Start

### 1.1 프로젝트 개요

**목표**: OpenClaw에 장기 기억 기능을 추가하는 사이드카 시스템 구축

**핵심 원칙**:
- ✅ Zero-Core-Modification (OpenClaw 코드 수정 없음)
- ✅ Sidecar Pattern (독립 프로세스)
- ✅ 90% 토큰 절약
- ✅ 90일+ 메모리 유지

### 1.2 MVP 범위 (Phase 1 - 4주)

```
주요 구성 요소:
┌─────────────────────────────────────────────────────┐
│                                                     │
│  1. FileWatcher  → 사용자 노트 디렉토리 감시        │
│  2. Observer     → LLM 기반 정보 추출 (선택사항)   │
│  3. MemoryWriter → Memory 파일 생성                 │
│  4. Setup Wizard → 인터랙티브 설정                  │
│                                                     │
│  통합: OpenClaw Memory 시스템                       │
│  (~/.openclaw/workspace/memory/*.md)                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 1.3 첫 주 목표

**Week 1 (Day 1-7)**: 기본 파이프라인 구축
- Day 1-2: 개발 환경 설정 + FileWatcher 프로토타입
- Day 3-4: Memory 파일 작성 로직
- Day 5-6: OpenClaw 통합 테스트
- Day 7: 검토 및 리팩토링

---

## 2. 현재 상태 및 우선순위

### 2.1 문서 현황

| 문서 | 상태 | 내용 |
|------|------|------|
| BRD.md | ✅ 완료 | 비즈니스 요구사항 |
| PRD.md | ✅ 완료 | 제품 요구사항 |
| Tech_Spec.md | ✅ 완료 | 기술 명세 |
| Tasks.md | ✅ 완료 | 상세 작업 계획 (100+ story points) |
| CHANGES.md | ✅ 완료 | OpenClaw 분석 후 변경 사항 |
| setup_wizard_example.py | ✅ 완료 | Setup wizard 예제 코드 |

### 2.2 아키텍처 변경 사항 (중요!)

**CHANGES.md 주요 내용**:
- ❌ 중앙 로그 파일 (chat.log) 없음 → Session Transcript 또는 Webhook 사용
- ✅ OpenClaw 내장 Memory 시스템 활용 (SQLite + Vector + FTS5)
- ✅ 외부 ChromaDB는 선택사항 (우선순위 낮춤)
- ✅ Memory 파일 방식이 가장 간단 (~/.openclaw/workspace/memory/)

### 2.3 우선순위 조정

#### P0 (Must Have - Sprint 1)
1. **FileWatcher**: 사용자 디렉토리 감시 (~/Documents/notes)
2. **MemoryWriter**: Markdown 파일 작성 (~/.openclaw/workspace/memory/)
3. **Basic Integration**: OpenClaw 자동 인덱싱 확인
4. **Setup Wizard**: 기본 설정 TUI

#### P1 (Should Have - Sprint 2)
5. **Observer**: LLM 기반 정보 추출
6. **Webhook**: 실시간 알림
7. **Error Handling**: Retry 정책

#### P2 (Nice to Have - Phase 2+)
8. ChromaDB 통합
9. Obsidian 연동
10. Reflector 압축

---

## 3. Sprint 1: 즉시 시작 가능한 작업

### 3.1 Sprint 1 목표 (Week 1-2)

**목표**: 기본 파이프라인 구축 및 OpenClaw 통합

**Deliverables**:
- [ ] FileWatcher 동작 (사용자 디렉토리 → Memory 디렉토리)
- [ ] MemoryWriter 동작 (Markdown 파일 생성)
- [ ] OpenClaw Memory 자동 인덱싱 확인
- [ ] Setup Wizard 기본 기능

**Story Points**: 35 (Week 1-2, 2명 기준)

---

## 4. 개발 환경 설정

### 4.1 필수 소프트웨어

#### OpenClaw 설치
```bash
# OpenClaw 공식 설치 방법 따르기
# https://github.com/openclaw/openclaw

# 설치 확인
openclaw --version

# 설정 파일 위치 확인
ls ~/.openclaw/openclaw.json
```

#### Python 환경
```bash
# Python 3.8+ 필요
python3 --version

# 가상 환경 생성
cd ~/GitHub/Oc-Memory
python3 -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# 기본 의존성 설치
pip install watchdog pyyaml python-dotenv questionary
```

### 4.2 프로젝트 구조 생성

```bash
# 프로젝트 디렉토리 구조
mkdir -p lib tests docs memory_db

# 기본 파일 생성
touch lib/__init__.py
touch lib/file_watcher.py
touch lib/memory_writer.py
touch lib/config.py
touch tests/__init__.py
touch setup.py
touch memory_observer.py
touch config.yaml
touch .env
touch .gitignore

# .gitignore 설정
cat > .gitignore << 'EOF'
# Python
__pycache__/
*.py[cod]
*$py.class
venv/
.pytest_cache/

# Environment
.env
*.env

# Memory DB
memory_db/
*.db

# Logs
*.log

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db
EOF
```

### 4.3 OpenClaw 설정 확인

```bash
# OpenClaw Memory 디렉토리 확인
ls -la ~/.openclaw/workspace/memory/

# 없으면 생성
mkdir -p ~/.openclaw/workspace/memory/

# OpenClaw 설정 파일 확인
cat ~/.openclaw/openclaw.json | grep -A 5 "memory"
```

### 4.4 테스트 환경 검증

```bash
# 테스트 파일 생성
echo "# Test Memory File" > ~/.openclaw/workspace/memory/test.md
echo "This is a test memory entry." >> ~/.openclaw/workspace/memory/test.md

# OpenClaw에서 memory_search tool 사용해보기
# OpenClaw CLI에서:
# > Use memory_search tool to find "test memory"

# 자동 인덱싱 확인 (약 5초 대기 후)
ls -la ~/.openclaw/agents/main/memory.db
```

---

## 5. Week 1 작업 계획

### Day 1-2: FileWatcher 프로토타입

#### Task 5.1.1: 기본 FileWatcher 클래스 작성

**파일**: `lib/file_watcher.py`

**목표**: 사용자 디렉토리를 감시하여 새 파일 감지

**구현 체크리스트**:
- [ ] watchdog 라이브러리 임포트
- [ ] FileWatcher 클래스 기본 구조
- [ ] on_created 이벤트 핸들러
- [ ] on_modified 이벤트 핸들러
- [ ] 마크다운 파일 필터링 (*.md)
- [ ] 로깅 추가

**예상 시간**: 3-4시간

**코드 템플릿**:
```python
# lib/file_watcher.py
import logging
from pathlib import Path
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler


class MarkdownFileHandler(FileSystemEventHandler):
    """마크다운 파일 변경 이벤트 핸들러"""

    def __init__(self, memory_dir: Path, callback=None):
        self.memory_dir = memory_dir
        self.callback = callback
        self.logger = logging.getLogger(__name__)

    def on_created(self, event):
        """새 파일 생성 이벤트"""
        if event.is_directory:
            return

        file_path = Path(event.src_path)
        if file_path.suffix.lower() == '.md':
            self.logger.info(f"New markdown file detected: {file_path}")
            if self.callback:
                self.callback(file_path, event_type='created')

    def on_modified(self, event):
        """파일 수정 이벤트"""
        if event.is_directory:
            return

        file_path = Path(event.src_path)
        if file_path.suffix.lower() == '.md':
            self.logger.info(f"Markdown file modified: {file_path}")
            if self.callback:
                self.callback(file_path, event_type='modified')


class FileWatcher:
    """사용자 디렉토리 감시 및 Memory 디렉토리 동기화"""

    def __init__(self, watch_dirs: list[str], memory_dir: str, callback=None):
        """
        Args:
            watch_dirs: 감시할 디렉토리 리스트
            memory_dir: OpenClaw Memory 디렉토리 (~/.openclaw/workspace/memory/)
            callback: 파일 변경 시 호출할 콜백 함수
        """
        self.watch_dirs = [Path(d).expanduser() for d in watch_dirs]
        self.memory_dir = Path(memory_dir).expanduser()
        self.callback = callback
        self.observer = Observer()
        self.logger = logging.getLogger(__name__)

        # Memory 디렉토리 생성
        self.memory_dir.mkdir(parents=True, exist_ok=True)

    def start(self):
        """감시 시작"""
        handler = MarkdownFileHandler(self.memory_dir, self.callback)

        for watch_dir in self.watch_dirs:
            if not watch_dir.exists():
                self.logger.warning(f"Watch directory does not exist: {watch_dir}")
                continue

            self.logger.info(f"Watching directory: {watch_dir}")
            self.observer.schedule(handler, str(watch_dir), recursive=True)

        self.observer.start()
        self.logger.info("FileWatcher started")

    def stop(self):
        """감시 중지"""
        self.observer.stop()
        self.observer.join()
        self.logger.info("FileWatcher stopped")


# 테스트 코드
if __name__ == "__main__":
    import time

    logging.basicConfig(level=logging.INFO)

    def file_callback(file_path, event_type):
        print(f"[{event_type}] {file_path}")

    watcher = FileWatcher(
        watch_dirs=["~/Documents/notes", "~/Projects"],
        memory_dir="~/.openclaw/workspace/memory",
        callback=file_callback
    )

    watcher.start()

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        watcher.stop()
```

**검증 방법**:
```bash
# 테스트 실행
python lib/file_watcher.py

# 다른 터미널에서 테스트 파일 생성
echo "# Test" > ~/Documents/notes/test.md

# 로그 확인 (파일 감지되어야 함)
```

**Definition of Done**:
- [ ] 파일 생성 감지 성공
- [ ] 파일 수정 감지 성공
- [ ] 마크다운 파일만 필터링
- [ ] 로그 정상 출력

---

#### Task 5.1.2: MemoryWriter 클래스 작성

**파일**: `lib/memory_writer.py`

**목표**: Memory 파일을 OpenClaw Memory 디렉토리에 작성

**구현 체크리스트**:
- [ ] MemoryWriter 클래스 기본 구조
- [ ] copy_to_memory() 메서드
- [ ] 메타데이터 추가 (날짜, 카테고리 등)
- [ ] 파일명 충돌 처리
- [ ] 에러 핸들링

**예상 시간**: 3-4시간

**코드 템플릿**:
```python
# lib/memory_writer.py
import logging
import shutil
from datetime import datetime
from pathlib import Path


class MemoryWriter:
    """OpenClaw Memory 디렉토리에 파일 작성"""

    def __init__(self, memory_dir: str):
        """
        Args:
            memory_dir: OpenClaw Memory 디렉토리
        """
        self.memory_dir = Path(memory_dir).expanduser()
        self.logger = logging.getLogger(__name__)

        # Memory 디렉토리 생성
        self.memory_dir.mkdir(parents=True, exist_ok=True)

    def copy_to_memory(self, source_file: Path, category: str = None) -> Path:
        """
        파일을 Memory 디렉토리로 복사

        Args:
            source_file: 원본 파일 경로
            category: 카테고리 (optional)

        Returns:
            복사된 파일 경로
        """
        if not source_file.exists():
            raise FileNotFoundError(f"Source file not found: {source_file}")

        # 대상 파일명 생성
        if category:
            # 카테고리별 서브디렉토리
            target_dir = self.memory_dir / category
            target_dir.mkdir(exist_ok=True)
        else:
            target_dir = self.memory_dir

        target_file = target_dir / source_file.name

        # 파일명 충돌 처리
        if target_file.exists():
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            target_file = target_dir / f"{source_file.stem}_{timestamp}{source_file.suffix}"

        # 파일 복사 (메타데이터 보존)
        shutil.copy2(source_file, target_file)

        self.logger.info(f"Copied to memory: {target_file}")
        return target_file

    def write_memory_entry(self, content: str, filename: str, category: str = None) -> Path:
        """
        새 메모리 항목 작성

        Args:
            content: 파일 내용
            filename: 파일명
            category: 카테고리 (optional)

        Returns:
            작성된 파일 경로
        """
        if category:
            target_dir = self.memory_dir / category
            target_dir.mkdir(exist_ok=True)
        else:
            target_dir = self.memory_dir

        target_file = target_dir / filename

        # 파일 작성
        with open(target_file, 'w', encoding='utf-8') as f:
            f.write(content)

        self.logger.info(f"Created memory entry: {target_file}")
        return target_file

    def add_metadata(self, file_path: Path, metadata: dict) -> None:
        """
        파일에 YAML frontmatter 추가

        Args:
            file_path: 파일 경로
            metadata: 메타데이터 딕셔너리
        """
        # 기존 내용 읽기
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # YAML frontmatter 생성
        frontmatter = "---\n"
        for key, value in metadata.items():
            frontmatter += f"{key}: {value}\n"
        frontmatter += "---\n\n"

        # 기존 frontmatter 제거 (있다면)
        if content.startswith("---"):
            parts = content.split("---", 2)
            if len(parts) >= 3:
                content = parts[2].lstrip()

        # 새 내용 작성
        new_content = frontmatter + content
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)

        self.logger.info(f"Added metadata to: {file_path}")


# 테스트 코드
if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)

    writer = MemoryWriter("~/.openclaw/workspace/memory")

    # 테스트 1: 파일 복사
    test_file = Path("~/Documents/notes/test.md").expanduser()
    if test_file.exists():
        result = writer.copy_to_memory(test_file, category="notes")
        print(f"Copied: {result}")

        # 메타데이터 추가
        writer.add_metadata(result, {
            "created": datetime.now().isoformat(),
            "category": "notes",
            "priority": "medium"
        })

    # 테스트 2: 새 항목 작성
    content = """# Project Context

## Overview
This is a test memory entry.

## Key Points
- Point 1
- Point 2
"""
    result = writer.write_memory_entry(
        content=content,
        filename="project_context.md",
        category="projects"
    )
    print(f"Created: {result}")
```

**검증 방법**:
```bash
# 테스트 실행
python lib/memory_writer.py

# Memory 디렉토리 확인
ls -la ~/.openclaw/workspace/memory/

# 파일 내용 확인
cat ~/.openclaw/workspace/memory/projects/project_context.md

# OpenClaw Memory DB 확인 (5초 후)
# OpenClaw에서 memory_search 사용
```

**Definition of Done**:
- [ ] 파일 복사 성공
- [ ] 메타데이터 추가 성공
- [ ] 파일명 충돌 처리
- [ ] OpenClaw 자동 인덱싱 확인

---

### Day 3-4: 통합 및 기본 파이프라인

#### Task 5.2.1: FileWatcher + MemoryWriter 통합

**파일**: `memory_observer.py` (메인 데몬)

**목표**: 전체 파이프라인 통합

**코드 템플릿**:
```python
# memory_observer.py
#!/usr/bin/env python3
"""
OC-Memory Observer Daemon
Watches user directories and syncs to OpenClaw Memory
"""

import logging
import signal
import sys
import time
from pathlib import Path

from lib.file_watcher import FileWatcher
from lib.memory_writer import MemoryWriter
from lib.config import load_config


class MemoryObserver:
    """Main daemon process"""

    def __init__(self, config_path: str = None):
        self.config_path = config_path or "config.yaml"
        self.config = load_config(self.config_path)
        self.logger = logging.getLogger(__name__)

        # Components
        self.memory_writer = MemoryWriter(
            memory_dir=self.config['memory']['dir']
        )

        self.file_watcher = FileWatcher(
            watch_dirs=self.config['watch']['dirs'],
            memory_dir=self.config['memory']['dir'],
            callback=self.on_file_change
        )

        self.running = False

    def on_file_change(self, file_path: Path, event_type: str):
        """파일 변경 이벤트 핸들러"""
        try:
            self.logger.info(f"Processing file: {file_path} ({event_type})")

            # 카테고리 자동 감지 (간단한 규칙 기반)
            category = self.detect_category(file_path)

            # Memory 디렉토리로 복사
            target_file = self.memory_writer.copy_to_memory(
                source_file=file_path,
                category=category
            )

            # 메타데이터 추가
            from datetime import datetime
            self.memory_writer.add_metadata(target_file, {
                "source": str(file_path),
                "synced_at": datetime.now().isoformat(),
                "category": category,
                "event_type": event_type
            })

            self.logger.info(f"Synced to memory: {target_file}")

        except Exception as e:
            self.logger.error(f"Error processing file {file_path}: {e}")

    def detect_category(self, file_path: Path) -> str:
        """파일 경로에서 카테고리 감지"""
        path_str = str(file_path).lower()

        if 'project' in path_str:
            return 'projects'
        elif 'note' in path_str:
            return 'notes'
        elif 'doc' in path_str:
            return 'documents'
        else:
            return 'general'

    def start(self):
        """Start daemon"""
        self.logger.info("Starting OC-Memory Observer...")

        # Signal handlers
        signal.signal(signal.SIGINT, self._signal_handler)
        signal.signal(signal.SIGTERM, self._signal_handler)

        # Start file watcher
        self.file_watcher.start()

        self.running = True
        self.logger.info("OC-Memory Observer started")

        # Main loop
        try:
            while self.running:
                time.sleep(1)
        except KeyboardInterrupt:
            pass

        self.stop()

    def stop(self):
        """Stop daemon"""
        self.logger.info("Stopping OC-Memory Observer...")
        self.running = False
        self.file_watcher.stop()
        self.logger.info("OC-Memory Observer stopped")

    def _signal_handler(self, signum, frame):
        """Signal handler"""
        self.logger.info(f"Received signal {signum}")
        self.stop()
        sys.exit(0)


def main():
    """Main entry point"""
    # Logging setup
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
        handlers=[
            logging.FileHandler('oc-memory.log'),
            logging.StreamHandler()
        ]
    )

    # Start daemon
    observer = MemoryObserver(config_path="config.yaml")
    observer.start()


if __name__ == "__main__":
    main()
```

**검증 방법**:
```bash
# 데몬 실행
python memory_observer.py

# 다른 터미널에서 테스트 파일 생성
echo "# Test Project" > ~/Documents/notes/my_project.md

# 로그 확인
tail -f oc-memory.log

# Memory 디렉토리 확인
ls -la ~/.openclaw/workspace/memory/projects/

# OpenClaw에서 검색 테스트
# memory_search "my project"
```

---

#### Task 5.2.2: Config 파일 작성

**파일**: `lib/config.py`, `config.yaml`

**구현**:

```python
# lib/config.py
import yaml
from pathlib import Path


def load_config(config_path: str) -> dict:
    """Load configuration from YAML file"""
    path = Path(config_path)

    if not path.exists():
        raise FileNotFoundError(f"Config file not found: {config_path}")

    with open(path, 'r') as f:
        config = yaml.safe_load(f)

    return config
```

```yaml
# config.yaml
# OC-Memory Configuration

watch:
  dirs:
    - ~/Documents/notes
    - ~/Projects
  recursive: true
  poll_interval: 1.0

memory:
  dir: ~/.openclaw/workspace/memory
  max_file_size: 10485760  # 10MB

logging:
  level: INFO
  file: oc-memory.log
```

---

### Day 5-6: OpenClaw 통합 테스트

#### Task 5.3.1: End-to-End 테스트

**목표**: 전체 파이프라인 검증

**테스트 시나리오**:

1. **기본 파일 동기화 테스트**
```bash
# 1. 데몬 시작
python memory_observer.py &

# 2. 테스트 파일 생성
cat > ~/Documents/notes/project_alpha.md << 'EOF'
# Project Alpha

## Objective
Build a new AI feature

## Requirements
- Real-time processing
- 99.9% accuracy
EOF

# 3. 파일 복사 확인 (즉시)
ls ~/.openclaw/workspace/memory/projects/

# 4. OpenClaw 인덱싱 확인 (5초 후)
sleep 5

# 5. OpenClaw에서 검색 테스트
# OpenClaw CLI:
# > Use memory_search to find "project alpha"
# Expected: project_alpha.md 결과 반환
```

2. **파일 수정 테스트**
```bash
# 1. 파일 수정
echo "## Update" >> ~/Documents/notes/project_alpha.md
echo "Added new requirement" >> ~/Documents/notes/project_alpha.md

# 2. 재동기화 확인
cat ~/.openclaw/workspace/memory/projects/project_alpha.md

# 3. OpenClaw 재인덱싱 확인 (5초 후)
# 검색 결과에 "Added new requirement" 포함되어야 함
```

3. **카테고리 자동 감지 테스트**
```bash
# 다양한 디렉토리에 파일 생성
echo "# Note 1" > ~/Documents/notes/general_note.md
echo "# Project 2" > ~/Projects/backend/api_design.md

# 카테고리별 분류 확인
tree ~/.openclaw/workspace/memory/
# Expected:
# memory/
# ├── notes/
# │   └── general_note.md
# └── projects/
#     ├── project_alpha.md
#     └── api_design.md
```

**Definition of Done**:
- [ ] 파일 생성 → 자동 동기화 확인
- [ ] 파일 수정 → 재동기화 확인
- [ ] OpenClaw memory_search 정상 작동
- [ ] 카테고리 자동 분류 확인

---

### Day 7: 리팩토링 및 문서화

#### Task 5.4.1: 코드 리팩토링

**체크리스트**:
- [ ] 에러 핸들링 강화
- [ ] 로깅 개선
- [ ] 타입 힌트 추가
- [ ] Docstring 작성
- [ ] 불필요한 코드 제거

#### Task 5.4.2: 기본 문서 작성

**작성할 문서**:
- [ ] README.md 업데이트
- [ ] INSTALL.md (설치 가이드)
- [ ] USAGE.md (사용법)
- [ ] TROUBLESHOOTING.md (문제 해결)

---

## 6. Week 2 작업 계획

### Day 8-10: Setup Wizard (TUI)

**참고**: `specs/setup_wizard_example.py` 이미 구현되어 있음

**작업**:
- [ ] 예제 코드 검토
- [ ] 실제 config.yaml 생성 로직 통합
- [ ] .env 파일 생성 및 권한 설정
- [ ] 유효성 검증 추가
- [ ] 테스트

**실행 방법**:
```bash
python setup.py
# 또는
oc-memory setup
```

### Day 11-12: 테스트 코드 작성

#### Unit Tests
```python
# tests/test_file_watcher.py
# tests/test_memory_writer.py
# tests/test_config.py
```

#### Integration Tests
```python
# tests/test_integration.py
```

**목표**: Coverage ≥ 80%

### Day 13-14: Sprint 1 검토 및 데모

**준비사항**:
- [ ] 모든 테스트 통과
- [ ] 문서 완성
- [ ] 데모 시나리오 준비
- [ ] Sprint 2 계획

---

## 7. 체크포인트 및 검증

### 7.1 Day 2 체크포인트

**검증 항목**:
- [ ] FileWatcher가 파일 생성을 감지하는가?
- [ ] MemoryWriter가 파일을 복사하는가?
- [ ] 로그가 정상 출력되는가?

**문제 발생 시**:
- watchdog 설치 확인: `pip install watchdog`
- 경로 권한 확인: `ls -la ~/Documents/notes`
- 로그 레벨 확인: `logging.DEBUG`로 변경

### 7.2 Day 5 체크포인트

**검증 항목**:
- [ ] OpenClaw가 파일을 자동 인덱싱하는가?
- [ ] memory_search가 정상 작동하는가?
- [ ] 전체 파이프라인이 5초 이내에 동작하는가?

**문제 발생 시**:
- OpenClaw 설정 확인: `cat ~/.openclaw/openclaw.json | grep memory`
- Memory DB 확인: `ls ~/.openclaw/agents/main/memory.db`
- 인덱싱 로그 확인: `tail -f ~/.openclaw/logs/indexing.log`

### 7.3 Week 1 완료 기준

**Must Have**:
- [ ] FileWatcher 동작 확인
- [ ] MemoryWriter 동작 확인
- [ ] OpenClaw 통합 테스트 통과
- [ ] 기본 문서 작성 완료

**Success Metrics**:
- 파일 동기화 지연: < 5초
- OpenClaw 검색 정확도: ≥ 90%
- 코드 테스트 커버리지: ≥ 70%

---

## 8. 다음 단계 (Sprint 2+)

### Sprint 2 (Week 3-4)
- [ ] Observer (LLM 기반 정보 추출)
- [ ] Webhook 통합
- [ ] Error Handling & Retry

### Sprint 3-4 (Week 5-7)
- [ ] ChromaDB 통합 (Optional)
- [ ] Reflector 압축
- [ ] TTL Management

### Sprint 5-6 (Week 8-11)
- [ ] Obsidian 통합
- [ ] 벤치마크
- [ ] Production 배포

---

## 9. 리소스 및 참고 자료

### 9.1 문서
- [BRD.md](./specs/BRD.md) - 비즈니스 요구사항
- [PRD.md](./specs/PRD.md) - 제품 요구사항
- [Tech_Spec.md](./specs/Tech_Spec.md) - 기술 명세
- [Tasks.md](./specs/Tasks.md) - 상세 작업 계획
- [CHANGES.md](./specs/CHANGES.md) - 아키텍처 변경 사항

### 9.2 외부 리소스
- [OpenClaw Documentation](https://github.com/openclaw/openclaw)
- [watchdog Library](https://python-watchdog.readthedocs.io/)
- [questionary Library](https://questionary.readthedocs.io/)

### 9.3 커뮤니케이션
- GitHub Issues: 버그 및 기능 요청
- 주간 스프린트 리뷰: 매주 금요일
- 일일 스탠드업: 선택사항

---

## 10. FAQ

### Q1: OpenClaw를 설치하지 않고 개발할 수 있나요?
**A**: FileWatcher와 MemoryWriter는 독립적으로 개발 가능합니다. 하지만 통합 테스트를 위해서는 OpenClaw 설치가 필요합니다.

### Q2: LLM API 키가 없어도 되나요?
**A**: Sprint 1 (기본 파이프라인)은 LLM 없이 동작합니다. Observer는 Sprint 2에서 구현됩니다.

### Q3: ChromaDB가 필수인가요?
**A**: 아니오. OpenClaw 내장 Memory 시스템을 활용하므로 외부 ChromaDB는 선택사항입니다.

### Q4: 개발 환경은 어떤 OS를 권장하나요?
**A**: macOS 또는 Linux를 권장합니다. Windows에서도 동작하지만 경로 처리 등에서 약간의 수정이 필요할 수 있습니다.

---

## 부록: 빠른 참조

### 명령어 치트시트
```bash
# 개발 환경 설정
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# 데몬 실행
python memory_observer.py

# Setup wizard
python setup.py

# 테스트
pytest tests/ -v --cov

# OpenClaw Memory 확인
ls ~/.openclaw/workspace/memory/
cat ~/.openclaw/openclaw.json | grep memory
```

### 디렉토리 구조
```
Oc-Memory/
├── lib/
│   ├── __init__.py
│   ├── file_watcher.py
│   ├── memory_writer.py
│   └── config.py
├── tests/
│   ├── test_file_watcher.py
│   └── test_memory_writer.py
├── specs/
│   ├── BRD.md
│   ├── PRD.md
│   ├── Tech_Spec.md
│   └── Tasks.md
├── memory_observer.py
├── setup.py
├── config.yaml
├── .env
├── .gitignore
└── README.md
```

---

**문서 버전**: 1.0
**작성자**: Claude Code
**작성일**: 2026-02-12
**다음 검토일**: Sprint 1 완료 후

🚀 **Ready to Start!** Day 1부터 바로 시작 가능합니다.
