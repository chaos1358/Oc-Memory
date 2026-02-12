# OC-Memory-Sidecar Task Implementation Plan
## OpenClaw 외장형 관찰 메모리 시스템 구현 계획

**버전**: 1.0
**작성일**: 2026-02-12
**작성자**: 아르고 (OpenClaw General Manager)
**상태**: Ready for Development

---

## 1. Development Roadmap

### Timeline Overview

```
Phase 1: MVP               Phase 2: Enhanced      Phase 3: Obsidian     Phase 4: Production
(Week 1-4)                 (Week 5-7)             (Week 8-9)            (Week 10-11)
├─ Sprint 1 (Week 1-2)    ├─ Sprint 3 (Week 5-6) ├─ Sprint 5 (Week 8-9)├─ Sprint 6 (Week 10-11)
├─ Sprint 2 (Week 3-4)    └─ Sprint 4 (Week 7)   │                     │
│                         │                       │                     │
Core Memory System        Semantic Search         Cloud Integration     Production Hardening
+ OpenClaw Integration    + TTL Management        + Advanced Features   + Testing + Docs
```

### Phase Breakdown

| Phase | Duration | Goal | Deliverables |
|-------|----------|------|--------------|
| **Phase 1: MVP** | 4주 | 기본 관찰 메모리 동작 | LogWatcher, Observer, active_memory.md, OpenClaw 연동 |
| **Phase 2: Enhanced** | 3주 | 의미 검색 및 수명 관리 | ChromaDB, Semantic Search, TTL Manager |
| **Phase 3: Obsidian** | 2주 | 클라우드 백업 연동 | Obsidian CLI, Dropbox Sync, 역방향 조회 |
| **Phase 4: Production** | 2주 | 프로덕션 준비 | 벤치마크, 최적화, 문서화, CI/CD |

---

## 2. Phase 1: MVP (Week 1-4)

### Epic 1.1: Core Memory System
**Goal**: OpenClaw Memory 시스템 연동 및 파일 동기화

#### User Story 1.1.1: FileWatcher 구현 (단순화됨)
**As a** 시스템 개발자
**I want** 사용자 노트 디렉토리를 감시하여 Memory 파일 동기화
**So that** OpenClaw가 자동으로 인덱싱하고 검색할 수 있다

**Acceptance Criteria**:
- [ ] 사용자 디렉토리 감시 기능 동작 (~/Documents/notes, ~/Projects)
- [ ] 새 파일 생성 시 `~/.openclaw/workspace/memory/` 에 자동 복사
- [ ] 파일 변경 시 동기화
- [ ] Webhook 알림 전송 (선택사항)

**Technical Tasks**:
- **Task 1.1.1.1**: FileWatcher 클래스 기본 구조 (단순화)
  - Priority: P0
  - Story Points: 2 (40% 감소)
  - Dependencies: None
  - Estimated Hours: 3h
  - Implementation:
    ```python
    # lib/file_watcher.py
    from watchdog.observers import Observer
    from watchdog.events import FileSystemEventHandler

    class FileWatcher:
        def __init__(self, watch_dirs, memory_dir):
            """
            watch_dirs: 감시할 디렉토리 리스트
            memory_dir: ~/.openclaw/workspace/memory/
            """
        def on_created(self, event):
            # 새 파일 생성 시 memory_dir로 복사
        def start(self):
            # 감시 시작
    ```

- **Task 1.1.1.2**: Memory 파일 동기화 로직
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 1.1.1.1
  - Estimated Hours: 3h
  - Implementation:
    - `shutil.copy2()` 사용하여 파일 복사
    - Markdown 파일만 필터링 (*.md)
    - 메타데이터 보존

- **Task 1.1.1.3**: Webhook 알림 (선택사항)
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 1.1.1.1
  - Estimated Hours: 3h
  - Implementation:
    - `POST http://localhost:18789/hooks/agent`
    - Bearer token 인증
    - 파일 경로 및 내용 요약 전송

- **Task 1.1.1.4**: 단위 테스트 작성
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 1.1.1.1-2
  - Estimated Hours: 3h
  - Test Cases:
    - 새 파일 감지
    - 상태 복구
    - 토큰 계산 정확도

**Definition of Done**:
- [ ] 모든 테스트 통과 (Coverage ≥ 80%)
- [ ] 로그 파일 변경 시 1초 이내 감지
- [ ] 프로세스 재시작 시 상태 복구

---

#### User Story 1.1.2: Observer Agent 구현
**As a** 시스템 개발자
**I want** 로그를 분석하여 구조화된 observation 생성
**So that** 의미 있는 정보만 메모리에 저장할 수 있다

**Acceptance Criteria**:
- [ ] LLM 호출하여 observation 추출
- [ ] 우선순위 분류 (🔴 🟡 🟢)
- [ ] 시간 정보 포함
- [ ] Mastra Observer 프롬프트 준수

**Technical Tasks**:
- **Task 1.1.2.1**: Observation 데이터 클래스 정의
  - Priority: P0
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h
  - Implementation:
    ```python
    @dataclass
    class Observation:
        id: str
        timestamp: datetime
        priority: str  # 'high', 'medium', 'low'
        category: str
        content: str
        metadata: dict
        def to_markdown(self) -> str
    ```

- **Task 1.1.2.2**: Observer 클래스 구현
  - Priority: P0
  - Story Points: 5
  - Dependencies: Task 1.1.2.1
  - Estimated Hours: 8h
  - Implementation:
    - OpenAI SDK 통합
    - System Prompt 로드 (Mastra 기반)
    - LLM 응답 파싱

- **Task 1.1.2.3**: Observer System Prompt 작성
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Content:
    - Mastra 프롬프트 기반
    - 우선순위 규칙 정의
    - 시간 앵커링 규칙

- **Task 1.1.2.4**: 응답 파싱 로직
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 1.1.2.2
  - Estimated Hours: 4h
  - Implementation:
    - 마크다운 라인 파싱
    - 이모지 우선순위 매핑
    - 시간 추출 및 파싱

- **Task 1.1.2.5**: 단위 테스트 및 통합 테스트
  - Priority: P1
  - Story Points: 4
  - Dependencies: All above
  - Estimated Hours: 6h

**Definition of Done**:
- [ ] LLM 호출 성공 (Google/OpenAI)
- [ ] Observation 포맷 정확도 ≥ 95%
- [ ] 응답 시간 ≤ 5초

---

#### User Story 1.1.3: Memory Merger 구현
**As a** 시스템 개발자
**I want** Observation을 active_memory.md에 병합
**So that** OpenClaw가 메모리 파일을 읽을 수 있다

**Acceptance Criteria**:
- [ ] active_memory.md 생성/업데이트
- [ ] 섹션별 내용 구성
- [ ] 토큰 제한 준수
- [ ] 파일 포맷 유지

**Technical Tasks**:
- **Task 1.1.3.1**: MemoryMerger 클래스 구조
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h

- **Task 1.1.3.2**: 섹션별 업데이트 로직
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 1.1.3.1
  - Estimated Hours: 6h
  - Sections:
    - Current Context
    - Observations Log
    - User Constraints
    - Completed Tasks
    - Critical Decisions

- **Task 1.1.3.3**: 토큰 제한 관리
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 1.1.3.2
  - Estimated Hours: 4h
  - Implementation:
    - 최대 토큰 초과 시 자동 압축
    - 오래된 observation 우선 제거

- **Task 1.1.3.4**: 단위 테스트
  - Priority: P1
  - Story Points: 3
  - Dependencies: All above
  - Estimated Hours: 4h

**Definition of Done**:
- [ ] active_memory.md 생성 성공
- [ ] 섹션 구조 정확
- [ ] 토큰 제한 준수 (≤ 30,000)

---

### Epic 1.2: OpenClaw Integration
**Goal**: OpenClaw와 연동하여 메모리 파일 읽기

#### User Story 1.2.1: System Prompt 수정
**As a** OpenClaw 사용자
**I want** Agent가 메모리 파일을 자동으로 읽도록
**So that** 이전 대화 맥락을 유지할 수 있다

**Technical Tasks**:
- **Task 1.2.1.1**: System Prompt 내용 작성
  - Priority: P0
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h

- **Task 1.2.1.2**: AGENTS.md 수정
  - Priority: P0
  - Story Points: 1
  - Dependencies: Task 1.2.1.1
  - Estimated Hours: 1h

- **Task 1.2.1.3**: 통합 테스트
  - Priority: P0
  - Story Points: 3
  - Dependencies: All above
  - Estimated Hours: 4h

**Definition of Done**:
- [ ] OpenClaw가 메모리 파일 읽기 성공
- [ ] 대화 맥락 유지 확인

---

### Epic 1.3: Setup Wizard (TUI)
**Goal**: 인터랙티브 설치 마법사 구현

#### User Story 1.3.1: Setup wizard 기본 구조
**As a** 사용자
**I want** 인터랙티브 설치 마법사
**So that** 쉽게 초기 설정을 완료할 수 있다

**Acceptance Criteria**:
- [ ] questionary 라이브러리 통합
- [ ] SetupWizard 클래스 구현
- [ ] 6단계 설정 플로우 동작
- [ ] config.yaml 자동 생성
- [ ] .env 파일 안전 관리

**Technical Tasks**:
- **Task 1.3.1.1**: questionary 통합
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Implementation:
    ```python
    import questionary
    # Interactive prompts for user input
    # Text, select, confirm, path inputs
    ```
  - Definition of Done:
    - [ ] questionary 설치 및 임포트 성공
    - [ ] 기본 프롬프트 동작 확인
    - [ ] 입력 유효성 검증 동작

- **Task 1.3.1.2**: SetupWizard 클래스 구현
  - Priority: P0
  - Story Points: 5
  - Dependencies: Task 1.3.1.1
  - Estimated Hours: 8h
  - Implementation:
    ```python
    # lib/setup_wizard.py
    class SetupWizard:
        def __init__(self)
        def run(self) -> dict
        def _welcome_screen(self)
        def _collect_configuration(self) -> dict
        def _save_configuration(self, config)
    ```
  - Definition of Done:
    - [ ] 클래스 구조 완성
    - [ ] 전체 플로우 동작
    - [ ] 설정 데이터 수집 성공

- **Task 1.3.1.3**: 6단계 설정 플로우
  - Priority: P0
  - Story Points: 8
  - Dependencies: Task 1.3.1.2
  - Estimated Hours: 12h
  - Implementation:
    - Step 1: OpenClaw 로그 경로 설정
    - Step 2: 메모리 저장 경로 설정
    - Step 3: LLM 제공자 선택 (Google/OpenAI)
    - Step 4: API 키 입력 및 검증
    - Step 5: Obsidian/Dropbox 선택 (Optional)
    - Step 6: 설정 요약 및 확인
  - Definition of Done:
    - [ ] 6단계 모두 구현
    - [ ] 단계별 입력 유효성 검증
    - [ ] 뒤로가기/취소 기능 동작
    - [ ] 설정 요약 화면 표시

- **Task 1.3.1.4**: Obsidian/Dropbox 선택 기능
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 1.3.1.3
  - Estimated Hours: 6h
  - Implementation:
    ```python
    def _setup_cloud_storage(self):
        choice = questionary.select(
            "Choose cloud storage",
            choices=["Obsidian", "Dropbox", "Skip"]
        ).ask()
        if choice == "Obsidian":
            return self._setup_obsidian()
        elif choice == "Dropbox":
            return self._setup_dropbox()
    ```
  - Definition of Done:
    - [ ] Obsidian vault 경로 설정
    - [ ] Dropbox 인증 플로우 구현
    - [ ] Skip 옵션 동작
    - [ ] 선택사항 config 반영

- **Task 1.3.1.5**: config.yaml 생성 로직
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 1.3.1.3
  - Estimated Hours: 4h
  - Implementation:
    ```python
    def _save_config_yaml(self, config: dict):
        # Generate config.yaml
        # Validate paths
        # Create directory structure
        # Write YAML file
    ```
  - Definition of Done:
    - [ ] config.yaml 생성 성공
    - [ ] 모든 필수 필드 포함
    - [ ] YAML 포맷 정확
    - [ ] 파일 권한 설정 (644)

- **Task 1.3.1.6**: .env 파일 관리
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 1.3.1.4
  - Estimated Hours: 6h
  - Implementation:
    ```python
    def _save_env_file(self, api_keys: dict):
        # Create .env file
        # Store API keys securely
        # Set file permissions (600)
        # Add to .gitignore
    ```
  - Definition of Done:
    - [ ] .env 파일 생성 성공
    - [ ] API 키 안전 저장 (600 권한)
    - [ ] .gitignore 자동 업데이트
    - [ ] 암호화 적용 (optional)

- **Task 1.3.1.7**: 유효성 검증
  - Priority: P0
  - Story Points: 5
  - Dependencies: Task 1.3.1.3
  - Estimated Hours: 8h
  - Implementation:
    ```python
    class ConfigValidator:
        def validate_path(self, path) -> bool
        def validate_api_key(self, key, provider) -> bool
        def test_api_connection(self, key, provider) -> bool
        def validate_permissions(self, path) -> bool
    ```
  - Definition of Done:
    - [ ] 경로 유효성 검증
    - [ ] API 키 포맷 검증
    - [ ] API 연결 테스트 성공
    - [ ] 파일 권한 확인

- **Task 1.3.1.8**: 에러 처리
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 1.3.1.7
  - Estimated Hours: 6h
  - Implementation:
    ```python
    try:
        config = wizard.run()
    except KeyboardInterrupt:
        print("Setup cancelled")
    except InvalidConfigError as e:
        print(f"Configuration error: {e}")
    except APIConnectionError as e:
        print(f"API connection failed: {e}")
    ```
  - Definition of Done:
    - [ ] 모든 예외 처리
    - [ ] 사용자 친화적 에러 메시지
    - [ ] 롤백 기능 구현
    - [ ] 재시도 옵션 제공

- **Task 1.3.1.9**: 단위 테스트
  - Priority: P1
  - Story Points: 6
  - Dependencies: All above
  - Estimated Hours: 9h
  - Test Cases:
    ```python
    # tests/test_setup_wizard.py
    def test_setup_wizard_flow()
    def test_config_validation()
    def test_api_key_validation()
    def test_file_creation()
    def test_error_handling()
    def test_rollback()
    ```
  - Definition of Done:
    - [ ] 모든 테스트 통과
    - [ ] Coverage ≥ 85%
    - [ ] 통합 테스트 통과
    - [ ] Mock API 테스트 구현

**Definition of Done**:
- [ ] 전체 설치 마법사 동작
- [ ] config.yaml 정확하게 생성
- [ ] .env 파일 안전하게 저장
- [ ] 모든 유효성 검증 통과
- [ ] 에러 처리 완벽
- [ ] 테스트 Coverage ≥ 85%

---

#### User Story 1.3.2: 6단계 설정 플로우
**As a** 사용자
**I want** 단계별 설정 가이드
**So that** 복잡한 설정을 쉽게 완료할 수 있다

**Acceptance Criteria**:
- [ ] 6단계 플로우 명확히 구분
- [ ] 각 단계별 도움말 제공
- [ ] 뒤로가기 기능 동작
- [ ] 진행률 표시

**Definition of Done**:
- [ ] 6단계 모두 구현
- [ ] 도움말 텍스트 완성
- [ ] 진행률 표시 동작
- [ ] UX 테스트 통과

---

#### User Story 1.3.3: Obsidian/Dropbox 선택 기능
**As a** 사용자
**I want** 클라우드 스토리지 선택
**So that** 내 환경에 맞는 백업 방법 사용

**Acceptance Criteria**:
- [ ] Obsidian/Dropbox/Skip 선택 가능
- [ ] Obsidian vault 경로 자동 감지
- [ ] Dropbox OAuth 인증 플로우
- [ ] 선택 없이 Skip 가능

**Definition of Done**:
- [ ] 3가지 옵션 모두 동작
- [ ] Obsidian 자동 감지 성공
- [ ] Dropbox 인증 완료
- [ ] Skip 시 로컬만 사용

---

#### User Story 1.3.4: API 키 안전 저장
**As a** 보안 담당자
**I want** API 키를 안전하게 저장
**So that** 민감 정보 유출 방지

**Acceptance Criteria**:
- [ ] .env 파일에 저장
- [ ] 파일 권한 600 설정
- [ ] .gitignore 자동 추가
- [ ] 평문 저장 금지

**Definition of Done**:
- [ ] .env 파일 생성
- [ ] 권한 설정 확인
- [ ] .gitignore 업데이트
- [ ] 보안 감사 통과

---

### Epic 1.4: Main Daemon Implementation
**Goal**: 메인 데몬 프로세스 구현

#### User Story 1.4.1: Memory Observer 데몬
**As a** 시스템 관리자
**I want** 백그라운드에서 자동 실행
**So that** 수동 개입 없이 메모리 관리

**Technical Tasks**:
- **Task 1.4.1.1**: memory_observer.py 메인 루프
  - Priority: P0
  - Story Points: 5
  - Dependencies: All Epic 1.1, Epic 1.3
  - Estimated Hours: 8h
  - Implementation:
    ```python
    class MemoryObserver:
        def __init__(self, config_path)
        def start(self)
        def stop(self)
        def _process_new_messages(self, lines, tokens)
    ```

- **Task 1.4.1.2**: Config 파일 파서
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 1.3.1.5
  - Estimated Hours: 3h

- **Task 1.4.1.3**: 로깅 시스템
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 1.4.1.1
  - Estimated Hours: 3h

- **Task 1.4.1.4**: 에러 핸들링 및 재시작
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 1.4.1.1
  - Estimated Hours: 4h

**Definition of Done**:
- [ ] 데몬 시작/중지 동작
- [ ] 에러 발생 시 자동 재시작
- [ ] 로그 파일 정상 기록

---

### Phase 1 Summary

**Total Story Points**: 100
- Epic 1.1 (Core Memory System): 40 points
- Epic 1.2 (OpenClaw Integration): 6 points
- Epic 1.3 (Setup Wizard TUI): 42 points
- Epic 1.4 (Main Daemon): 12 points

**Total Estimated Hours**: ~150 hours
**Team Velocity**: 25 points/week (assuming 2 developers)
**Duration**: 4 weeks

**Key Deliverables**:
1. Functional log watcher with state management
2. LLM-based observation extraction
3. Active memory file generation
4. Interactive setup wizard (TUI)
5. Secure configuration management
6. OpenClaw integration
7. Background daemon process

**Success Metrics**:
- [ ] Setup wizard completes in < 5 minutes
- [ ] Config validation accuracy: 100%
- [ ] API key storage security: 600 permissions
- [ ] Zero manual configuration required
- [ ] All unit tests pass with coverage ≥ 85%

---

## 3. Phase 2: Enhanced Features (Week 5-7)

### Epic 2.1: Plugin Hook Development
**Goal**: OpenClaw Plugin 개발로 동적 통합 구현

#### User Story 2.1.1: Plugin Hook 구현
**As a** 시스템 개발자
**I want** OpenClaw Plugin Hook을 개발하여 동적 Memory Context 주입
**So that** 실시간으로 메모리를 에이전트에게 제공할 수 있다

**Technical Tasks**:
- **Task 2.1.1.1**: Plugin 기본 구조 작성
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 5h
  - Implementation:
    ```javascript
    // ~/.openclaw/plugins/oc-memory/index.js
    module.exports = {
      name: "oc-memory-integration",
      version: "1.0.0",
      hooks: [
        {
          hookName: "before_agent_start",
          handler: async (event, ctx) => {
            // 동적 메모리 로드 및 주입
          }
        }
      ]
    };
    ```

- **Task 2.1.1.2**: before_agent_start Hook 구현
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 2.1.1.1
  - Estimated Hours: 6h
  - Implementation:
    - 최근 Memory 파일 읽기 (top 5)
    - Markdown 포맷팅
    - `prependContext` 반환

- **Task 2.1.1.3**: after_tool_call Hook 구현
  - Priority: P1
  - Story Points: 3
  - Dependencies: Task 2.1.1.1
  - Estimated Hours: 5h
  - Implementation:
    - write_file tool 감지
    - Markdown 파일 자동 복사
    - Memory 디렉토리 동기화
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 2.1.1.1
  - Estimated Hours: 4h

- **Task 2.1.1.4**: 검색 기능 구현
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 2.1.1.3
  - Estimated Hours: 6h

- **Task 2.1.1.5**: 단위 테스트 및 성능 테스트
  - Priority: P1
  - Story Points: 4
  - Dependencies: All above
  - Estimated Hours: 6h

**Definition of Done**:
- [ ] ChromaDB 정상 동작
- [ ] 검색 응답 시간 ≤ 500ms
- [ ] 검색 정확도 ≥ 85%

---

#### User Story 2.1.2: CLI 검색 인터페이스
**As a** 사용자
**I want** CLI로 메모리 검색
**So that** 과거 정보를 빠르게 찾을 수 있다

**Technical Tasks**:
- **Task 2.1.2.1**: CLI 명령어 구현
  - Priority: P1
  - Story Points: 3
  - Dependencies: Task 2.1.1.4
  - Estimated Hours: 4h
  - Commands:
    ```bash
    oc-memory search "query"
    oc-memory search --priority high "query"
    oc-memory search --date 2026-02-12 "query"
    ```

- **Task 2.1.2.2**: 결과 포맷팅
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 2.1.2.1
  - Estimated Hours: 3h

**Definition of Done**:
- [ ] 검색 명령어 동작
- [ ] 결과 정확성 ≥ 90%

---

### Epic 2.2: Reflector Implementation
**Goal**: Observation 압축 및 장기 기억화

#### User Story 2.2.1: Reflector Agent
**As a** 시스템 개발자
**I want** Observation을 압축하여 토큰 절약
**So that** 장기 메모리를 효율적으로 관리

**Technical Tasks**:
- **Task 2.2.1.1**: Reflector 클래스 구현
  - Priority: P0
  - Story Points: 5
  - Dependencies: None
  - Estimated Hours: 8h
  - Implementation:
    ```python
    class Reflector:
        def __init__(self, model, api_key)
        def reflect(self, observations) -> str
        def _build_reflection_prompt(self, observations)
        def _compress(self, content, level)
    ```

- **Task 2.2.1.2**: Reflector System Prompt 작성
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Content: Mastra 기반 압축 프롬프트

- **Task 2.2.1.3**: 압축 레벨 로직
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 2.2.1.1
  - Estimated Hours: 6h
  - Levels:
    - Level 0: 압축 없음
    - Level 1: 8/10 detail
    - Level 2: 6/10 detail

- **Task 2.2.1.4**: Reflection 트리거 로직
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 2.2.1.1
  - Estimated Hours: 4h
  - Trigger: observation_tokens ≥ 40,000

- **Task 2.2.1.5**: 단위 테스트 및 압축률 벤치마크
  - Priority: P1
  - Story Points: 4
  - Dependencies: All above
  - Estimated Hours: 6h

**Definition of Done**:
- [ ] 압축률 ≥ 5x
- [ ] 토큰 절약률 ≥ 80%
- [ ] 정보 손실 ≤ 10%

---

### Epic 2.3: TTL Management
**Goal**: 메모리 수명 주기 관리

#### User Story 2.3.1: 3-Tier Memory System
**As a** 시스템 관리자
**I want** 메모리를 Hot/Warm/Cold로 자동 이동
**So that** 저장 공간과 성능을 최적화

**Technical Tasks**:
- **Task 2.3.1.1**: TTLManager 클래스 구현
  - Priority: P0
  - Story Points: 5
  - Dependencies: MemoryStore
  - Estimated Hours: 8h
  - Implementation:
    ```python
    class TTLManager:
        def __init__(self, memory_store, archive_path, hot_ttl, warm_ttl)
        def check_and_archive(self) -> dict
        def _archive_to_warm(self, observation_id)
        def _archive_to_cold(self, file_path)
    ```

- **Task 2.3.1.2**: Hot → Warm 자동 이동
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 2.3.1.1
  - Estimated Hours: 4h
  - Logic: 90일 경과 시 Markdown 파일로 이동

- **Task 2.3.1.3**: Warm → Cold 수동 승인
  - Priority: P1
  - Story Points: 3
  - Dependencies: Task 2.3.1.2
  - Estimated Hours: 4h
  - Logic: 1년 경과 시 사용자 승인 요청

- **Task 2.3.1.4**: Cron 스케줄러 통합
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 2.3.1.1
  - Estimated Hours: 3h
  - Schedule: 매일 새벽 3시 실행

- **Task 2.3.1.5**: 단위 테스트
  - Priority: P1
  - Story Points: 3
  - Dependencies: All above
  - Estimated Hours: 4h

**Definition of Done**:
- [ ] TTL 정책 정상 동작
- [ ] 데이터 손실 0%
- [ ] 아카이브 성공률 100%

---

### Epic 2.4: Error Handling & Recovery
**Goal**: LLM API 실패 시 자동 재시도 및 알림 시스템 구축

#### User Story 2.4.1: Retry Policy 구현
**As a** 시스템 개발자
**I want** LLM API 호출 실패 시 자동 재시도
**So that** 일시적 네트워크 장애를 자동으로 복구

**Acceptance Criteria**:
- [ ] 3회 재시도 동작
- [ ] 지수 백오프 (2초, 4초, 8초)
- [ ] tenacity 라이브러리 사용
- [ ] 각 재시도 로그 기록

**Technical Tasks**:
- **Task 2.4.1.1**: LLMRetryPolicy 클래스 구현
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Implementation:
    ```python
    # lib/error_handler.py
    class LLMRetryPolicy:
        @retry(
            stop=stop_after_attempt(3),
            wait=wait_exponential(multiplier=1, min=2, max=8),
            reraise=True
        )
        def call_with_retry(self, llm_function, *args, **kwargs)
    ```

- **Task 2.4.1.2**: Observer/Reflector에 Retry 적용
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 2.4.1.1
  - Estimated Hours: 6h
  - Implementation:
    - ObserverWithRetry 클래스
    - ReflectorWithRetry 클래스
    - 기존 Observer/Reflector 래핑

- **Task 2.4.1.3**: 재시도 로깅 시스템
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 2.4.1.1
  - Estimated Hours: 3h
  - Implementation:
    - 각 재시도 attempt 로그
    - 최종 실패 로그
    - 재시도 통계 수집

- **Task 2.4.1.4**: 단위 테스트
  - Priority: P1
  - Story Points: 3
  - Dependencies: All above
  - Estimated Hours: 4h
  - Test Cases:
    - 1회 시도 성공
    - 2회 시도 성공
    - 3회 시도 성공
    - 3회 모두 실패
    - 지수 백오프 정확도

**Definition of Done**:
- [ ] 재시도 정책 동작
- [ ] 3회 재시도 확인
- [ ] 지수 백오프 검증
- [ ] 로그 정확성 확인

---

#### User Story 2.4.2: OpenClaw API 자동 탐지
**As a** 사용자
**I want** OpenClaw API를 자동으로 찾기
**So that** 수동 설정 없이 편리하게 사용

**Acceptance Criteria**:
- [ ] 4가지 탐지 방법 동작
- [ ] 우선순위 순서대로 시도
- [ ] 연결 테스트 성공
- [ ] TUI에서 수동 입력 가능

**Technical Tasks**:
- **Task 2.4.2.1**: OpenClawAPIDetector 클래스
  - Priority: P0
  - Story Points: 5
  - Dependencies: None
  - Estimated Hours: 8h
  - Implementation:
    ```python
    # lib/api_detector.py
    class OpenClawAPIDetector:
        def detect_api_endpoint(self) -> Optional[str]
        def _read_openclaw_config(self) -> Optional[str]
        def _scan_openclaw_process(self) -> Optional[str]
        def _test_connection(self, endpoint: str) -> bool
    ```
  - Detection Methods:
    1. OpenClaw config.yaml 파싱
    2. 환경 변수 (OPENCLAW_API_URL)
    3. 프로세스 포트 스캔 (psutil)
    4. 기본 포트 테스트 (8080, 8000, 3000)

- **Task 2.4.2.2**: Config 파일 파서
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 2.4.2.1
  - Estimated Hours: 3h
  - Implementation:
    - ~/.openclaw/config.yaml 읽기
    - http_api.endpoint 추출

- **Task 2.4.2.3**: 프로세스 스캔 (psutil)
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 2.4.2.1
  - Estimated Hours: 4h
  - Implementation:
    - openclaw 프로세스 찾기
    - LISTEN 포트 추출

- **Task 2.4.2.4**: 연결 테스트
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 2.4.2.1
  - Estimated Hours: 3h
  - Implementation:
    - GET /health 호출
    - 타임아웃 2초

- **Task 2.4.2.5**: TUI 통합
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 2.4.2.1, Epic 1.3
  - Estimated Hours: 6h
  - Implementation:
    - Step 5: 에러 알림 설정
    - 자동 탐지 시도 및 결과 표시
    - 수동 입력 폼
    - 연결 테스트 UI

- **Task 2.4.2.6**: 단위 테스트
  - Priority: P1
  - Story Points: 3
  - Dependencies: All above
  - Estimated Hours: 4h
  - Test Cases:
    - Config 파일 파싱
    - 환경 변수 탐지
    - 프로세스 스캔
    - 기본 포트 테스트
    - 연결 테스트

**Definition of Done**:
- [ ] 자동 탐지 성공률 ≥ 80%
- [ ] 모든 탐지 방법 동작
- [ ] 연결 테스트 정확
- [ ] TUI 통합 완료

---

#### User Story 2.4.3: HTTP API Hook 알림
**As a** 사용자
**I want** LLM 압축 실패 시 알림 받기
**So that** 장애를 빠르게 인지하고 대응

**Acceptance Criteria**:
- [ ] HTTP POST 요청 성공
- [ ] JSON Payload 정확
- [ ] OpenClaw 수신 확인
- [ ] Telegram 알림 수신

**Technical Tasks**:
- **Task 2.4.3.1**: ErrorNotifier 클래스 구현
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 2.4.2.1
  - Estimated Hours: 6h
  - Implementation:
    ```python
    # lib/error_notifier.py
    class ErrorNotifier:
        def __init__(self, config)
        def notify_openclaw(self, error_details: dict) -> bool
    ```
  - Payload:
    ```json
    {
      "source": "oc-memory",
      "event": "compression_failed",
      "severity": "high",
      "timestamp": "ISO8601",
      "details": {
        "component": "Observer/Reflector",
        "error_type": "TimeoutError",
        "error_message": "...",
        "retry_count": 3,
        "token_count": 35000
      },
      "action_required": true
    }
    ```

- **Task 2.4.3.2**: Observer/Reflector 통합
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 2.4.3.1, Task 2.4.1.2
  - Estimated Hours: 4h
  - Implementation:
    - 3회 재시도 실패 시 notify_openclaw 호출
    - 에러 상세 정보 수집
    - 예외 재발생 (raise)

- **Task 2.4.3.3**: 알림 전송 재시도 (Optional)
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 2.4.3.1
  - Estimated Hours: 3h
  - Implementation:
    - 알림 전송 실패 시 1회 재시도
    - 최종 실패 시 로그만 기록

- **Task 2.4.3.4**: 단위 테스트 및 통합 테스트
  - Priority: P1
  - Story Points: 4
  - Dependencies: All above
  - Estimated Hours: 6h
  - Test Cases:
    - HTTP POST 성공
    - Payload 포맷 검증
    - 네트워크 오류 처리
    - End-to-end 테스트 (Mock OpenClaw)

**Definition of Done**:
- [ ] HTTP API 호출 성공
- [ ] Payload 정확성 확인
- [ ] 에러 처리 완벽
- [ ] 통합 테스트 통과

---

#### User Story 2.4.4: Config 파일 스키마 확장
**As a** 시스템 개발자
**I want** 에러 알림 설정을 config.yaml에 저장
**So that** 설정 관리가 용이

**Technical Tasks**:
- **Task 2.4.4.1**: config.yaml 스키마 확장
  - Priority: P0
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h
  - Schema:
    ```yaml
    error_notification:
      openclaw_api: "http://localhost:8080"
      retry_count: 3
      retry_delays: [2, 4, 8]
      connection_verified: true
    ```

- **Task 2.4.4.2**: Config 파서 업데이트
  - Priority: P0
  - Story Points: 2
  - Dependencies: Task 2.4.4.1
  - Estimated Hours: 2h

- **Task 2.4.4.3**: Setup Wizard 통합
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 2.4.4.1, Task 2.4.2.5
  - Estimated Hours: 4h

**Definition of Done**:
- [ ] 스키마 문서화
- [ ] Config 읽기/쓰기 동작
- [ ] Setup Wizard 반영

---

### Epic 2.4 Summary

**Total Story Points**: 45
- US 2.4.1 (Retry Policy): 12 points
- US 2.4.2 (API Auto-detection): 19 points
- US 2.4.3 (HTTP API Hook): 13 points
- US 2.4.4 (Config Schema): 7 points

**Total Estimated Hours**: ~70 hours
**Dependencies**:
- Epic 1.3 (Setup Wizard) - TUI 통합
- Epic 1.1 (Observer) - Retry 적용
- Epic 2.2 (Reflector) - Retry 적용

**Key Deliverables**:
1. LLM API 재시도 정책 (3회, 지수 백오프)
2. OpenClaw API 자동 탐지 (4가지 방법)
3. HTTP API Hook 알림 시스템
4. TUI 에러 알림 설정
5. Config 파일 스키마 확장

**Success Metrics**:
- [ ] 재시도 성공률 ≥ 70% (일시적 장애 복구)
- [ ] 자동 탐지 성공률 ≥ 80%
- [ ] 알림 전송 성공률 ≥ 95%
- [ ] Zero-Core-Modification 원칙 유지
- [ ] No Fallback Strategy (품질 저하 방지)

**Definition of Done**:
- [ ] 모든 User Story 완료
- [ ] 단위 테스트 Coverage ≥ 85%
- [ ] 통합 테스트 통과
- [ ] TUI 통합 완료
- [ ] 문서 업데이트 (PRD, Tech_Spec)
- [ ] Code Review 승인

---

### Phase 2 Summary

**Total Story Points**: 103
- Epic 2.1 (ChromaDB Integration): 18 points
- Epic 2.2 (Reflector Implementation): 19 points
- Epic 2.3 (TTL Management): 16 points
- Epic 2.4 (Error Handling & Recovery): 45 points
- Phase 2 Unassigned Tasks: 5 points

**Total Estimated Hours**: ~160 hours
**Team Velocity**: 25 points/week (assuming 2 developers)
**Duration**: 3 weeks

**Key Deliverables**:
1. ChromaDB semantic search
2. LLM-based compression (Reflector)
3. 3-Tier memory management (Hot/Warm/Cold)
4. Automatic retry policy
5. OpenClaw API auto-detection
6. HTTP API notification system
7. Error recovery mechanisms

**Success Metrics**:
- [ ] Semantic search accuracy ≥ 85%
- [ ] Compression ratio ≥ 5x
- [ ] Token savings ≥ 90%
- [ ] TTL policy functioning correctly
- [ ] Retry success rate ≥ 70%
- [ ] API auto-detection success rate ≥ 80%
- [ ] Notification delivery success rate ≥ 95%

---

## 4. Phase 3: Obsidian Integration (Week 8-9)

### Epic 3.1: Obsidian CLI Integration
**Goal**: Obsidian Vault에 Cold Memory 저장

#### User Story 3.1.1: Obsidian CLI 연동
**As a** 사용자
**I want** 오래된 메모리를 Obsidian에 자동 백업
**So that** 영구 보존 및 검색 가능

**Technical Tasks**:
- **Task 3.1.1.1**: Obsidian CLI 래퍼 클래스
  - Priority: P1
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Implementation:
    ```python
    class ObsidianClient:
        def __init__(self, vault_path, cli_path)
        def create_note(self, title, content, folder)
        def search_notes(self, query)
        def get_note(self, note_path)
    ```

- **Task 3.1.1.2**: Cold Memory 포맷 정의
  - Priority: P1
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h
  - Format: Obsidian-friendly Markdown

- **Task 3.1.1.3**: 아카이브 로직 통합
  - Priority: P1
  - Story Points: 3
  - Dependencies: Task 3.1.1.1, TTLManager
  - Estimated Hours: 4h

- **Task 3.1.1.4**: 단위 테스트
  - Priority: P2
  - Story Points: 2
  - Dependencies: All above
  - Estimated Hours: 3h

**Definition of Done**:
- [ ] Obsidian에 노트 생성 성공
- [ ] 메타데이터 보존
- [ ] 파일 구조 정확

---

### Epic 3.2: Dropbox Sync (Optional)
**Goal**: Obsidian Vault를 Dropbox로 자동 동기화

#### User Story 3.2.1: Dropbox 연동
**As a** 사용자
**I want** Obsidian Vault를 클라우드에 백업
**So that** 여러 기기에서 접근 가능

**Technical Tasks**:
- **Task 3.2.1.1**: Dropbox API 클라이언트
  - Priority: P2
  - Story Points: 4
  - Dependencies: None
  - Estimated Hours: 6h

- **Task 3.2.1.2**: 자동 동기화 로직
  - Priority: P2
  - Story Points: 3
  - Dependencies: Task 3.2.1.1
  - Estimated Hours: 4h

- **Task 3.2.1.3**: 역방향 조회 (Dropbox → Local)
  - Priority: P2
  - Story Points: 4
  - Dependencies: Task 3.2.1.1
  - Estimated Hours: 6h

**Definition of Done**:
- [ ] Dropbox 업로드 성공
- [ ] 충돌 해결 로직 동작
- [ ] 검색 API 정상 동작

---

### Epic 3.3: Advanced Search
**Goal**: 고급 검색 기능

#### User Story 3.3.1: 통합 검색
**As a** 사용자
**I want** Hot/Warm/Cold 메모리를 한 번에 검색
**So that** 모든 기록을 빠르게 찾을 수 있다

**Technical Tasks**:
- **Task 3.3.1.1**: 통합 검색 엔진
  - Priority: P1
  - Story Points: 5
  - Dependencies: All storage layers
  - Estimated Hours: 8h
  - Search Order:
    1. ChromaDB (semantic)
    2. Markdown files (grep)
    3. Obsidian (CLI search)

- **Task 3.3.1.2**: 결과 병합 및 랭킹
  - Priority: P1
  - Story Points: 3
  - Dependencies: Task 3.3.1.1
  - Estimated Hours: 4h

- **Task 3.3.1.3**: CLI 명령어 확장
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 3.3.1.2
  - Estimated Hours: 3h
  - Command:
    ```bash
    oc-memory search --all "query"
    oc-memory search --tier hot "query"
    oc-memory search --tier warm "query"
    oc-memory search --tier cold "query"
    ```

**Definition of Done**:
- [ ] 통합 검색 동작
- [ ] 검색 속도 ≤ 2초
- [ ] 결과 정확도 ≥ 85%

---

## 5. Phase 4: Production Ready (Week 10-11)

### Epic 4.1: Testing & Benchmarking
**Goal**: 프로덕션 품질 검증

#### User Story 4.1.1: LongMemEval 벤치마크
**As a** QA 엔지니어
**I want** 표준 벤치마크로 성능 측정
**So that** 품질을 객관적으로 평가

**Technical Tasks**:
- **Task 4.1.1.1**: LongMemEval 데이터셋 준비
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h

- **Task 4.1.1.2**: 벤치마크 스크립트 작성
  - Priority: P0
  - Story Points: 4
  - Dependencies: Task 4.1.1.1
  - Estimated Hours: 6h

- **Task 4.1.1.3**: 성능 분석 및 리포트
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 4.1.1.2
  - Estimated Hours: 4h
  - Metrics:
    - Retrieval accuracy
    - Token savings
    - Compression ratio
    - Latency

**Definition of Done**:
- [ ] LongMemEval 점수 ≥ 85%
- [ ] 토큰 절약률 ≥ 90%
- [ ] 응답 지연 ≤ 1초

---

#### User Story 4.1.2: 부하 테스트
**As a** QA 엔지니어
**I want** 고부하 상황에서 안정성 검증
**So that** 프로덕션 환경에서 신뢰 가능

**Technical Tasks**:
- **Task 4.1.2.1**: 부하 테스트 시나리오 작성
  - Priority: P0
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 3h
  - Scenarios:
    - 1000+ observations
    - 100+ concurrent searches
    - 24시간 연속 실행

- **Task 4.1.2.2**: 부하 테스트 실행
  - Priority: P0
  - Story Points: 3
  - Dependencies: Task 4.1.2.1
  - Estimated Hours: 4h

- **Task 4.1.2.3**: 성능 병목 분석 및 최적화
  - Priority: P0
  - Story Points: 5
  - Dependencies: Task 4.1.2.2
  - Estimated Hours: 8h

**Definition of Done**:
- [ ] 메모리 누수 없음
- [ ] CPU 사용률 < 10%
- [ ] 24시간 안정 동작

---

### Epic 4.2: CI/CD Pipeline
**Goal**: 자동화된 테스트 및 배포

#### User Story 4.2.1: GitHub Actions
**As a** DevOps 엔지니어
**I want** PR마다 자동 테스트
**So that** 코드 품질 유지

**Technical Tasks**:
- **Task 4.2.1.1**: GitHub Actions 워크플로우 작성
  - Priority: P0
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Workflow:
    - Unit tests
    - Integration tests
    - Linting (black, mypy)
    - Coverage report

- **Task 4.2.1.2**: Pre-commit hooks
  - Priority: P1
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h

- **Task 4.2.1.3**: 자동 배포 스크립트
  - Priority: P1
  - Story Points: 3
  - Dependencies: Task 4.2.1.1
  - Estimated Hours: 4h

**Definition of Done**:
- [ ] PR 자동 테스트 동작
- [ ] Coverage ≥ 80%
- [ ] 배포 스크립트 정상 동작

---

### Epic 4.3: Documentation
**Goal**: 완전한 문서화

#### User Story 4.3.1: 사용자 문서
**As a** 사용자
**I want** 명확한 설치 및 사용 가이드
**So that** 쉽게 시작할 수 있다

**Technical Tasks**:
- **Task 4.3.1.1**: README.md 작성
  - Priority: P0
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 3h
  - Sections:
    - Quick Start
    - Installation
    - Configuration
    - Usage Examples
    - Troubleshooting

- **Task 4.3.1.2**: API 문서 생성
  - Priority: P1
  - Story Points: 3
  - Dependencies: None
  - Estimated Hours: 4h
  - Tool: Sphinx or MkDocs

- **Task 4.3.1.3**: 튜토리얼 비디오/스크린샷
  - Priority: P2
  - Story Points: 2
  - Dependencies: Task 4.3.1.1
  - Estimated Hours: 3h

**Definition of Done**:
- [ ] README 완성
- [ ] API 문서 완성
- [ ] 예제 코드 동작 확인

---

### Epic 4.4: Deployment
**Goal**: 프로덕션 환경 배포

#### User Story 4.4.1: 시스템 서비스 등록
**As a** 시스템 관리자
**I want** 부팅 시 자동 시작
**So that** 수동 관리 불필요

**Technical Tasks**:
- **Task 4.4.1.1**: macOS LaunchAgent 작성
  - Priority: P0
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h
  - File: `com.openclaw.oc-memory.plist`

- **Task 4.4.1.2**: Linux systemd 서비스 작성
  - Priority: P1
  - Story Points: 2
  - Dependencies: None
  - Estimated Hours: 2h
  - File: `oc-memory.service`

- **Task 4.4.1.3**: 설치 스크립트
  - Priority: P0
  - Story Points: 3
  - Dependencies: All above
  - Estimated Hours: 4h
  - Script: `install.sh`

- **Task 4.4.1.4**: 언인스톨 스크립트
  - Priority: P1
  - Story Points: 2
  - Dependencies: Task 4.4.1.3
  - Estimated Hours: 2h
  - Script: `uninstall.sh`

**Definition of Done**:
- [ ] 자동 시작 동작
- [ ] 설치 스크립트 정상 동작
- [ ] 언인스톨 클린업 완료

---

## 6. Sprint Planning

### Sprint 1 (Week 1-2): Foundation
**Goal**: 핵심 파이프라인 구축 및 설치 마법사

**Deliverables**:
- [ ] LogWatcher 동작
- [ ] Observer 동작
- [ ] active_memory.md 생성
- [ ] Setup Wizard 완성

**Tasks**:
- Epic 1.1 (US 1.1.1, 1.1.2, 1.1.3)
- Epic 1.3 (US 1.3.1, 1.3.2, 1.3.3, 1.3.4)
- Total Story Points: 35 + 42 = 77

**Sprint Review Criteria**:
- 로그 감시 성공
- LLM 호출 성공
- 메모리 파일 생성 성공
- 설치 마법사 동작 확인
- config.yaml 및 .env 파일 생성 성공

---

### Sprint 2 (Week 3-4): Integration
**Goal**: OpenClaw 연동 완료

**Deliverables**:
- [ ] OpenClaw System Prompt 연동
- [ ] 메인 데몬 동작
- [ ] End-to-end 테스트 통과

**Tasks**:
- Epic 1.2, 1.4
- Total Story Points: 20

**Sprint Review Criteria**:
- OpenClaw가 메모리 읽기 성공
- 데몬 백그라운드 실행
- 대화 맥락 유지 확인

---

### Sprint 3 (Week 5-6): Semantic Search
**Goal**: ChromaDB 및 Reflector 구현

**Deliverables**:
- [ ] ChromaDB 통합
- [ ] Semantic Search 동작
- [ ] Reflector 압축 동작

**Tasks**:
- Epic 2.1, 2.2
- Total Story Points: 42

**Sprint Review Criteria**:
- 검색 정확도 ≥ 85%
- 압축률 ≥ 5x
- 토큰 절약률 ≥ 80%

---

### Sprint 4 (Week 7): TTL Management & Error Handling
**Goal**: 메모리 수명 주기 관리 및 에러 복구 시스템 구축

**Deliverables**:
- [ ] 3-Tier Memory 동작
- [ ] Hot → Warm 자동 이동
- [ ] Cron 스케줄러 동작
- [ ] LLM API 재시도 정책 동작
- [ ] OpenClaw API 자동 탐지
- [ ] HTTP API 알림 시스템 구축

**Tasks**:
- Epic 2.3 (TTL Management)
- Epic 2.4 (Error Handling & Recovery)
- Total Story Points: 61

**Sprint Review Criteria**:
- TTL 정책 동작
- 데이터 손실 0%
- 아카이브 성공
- 재시도 정책 동작 (3회, 지수 백오프)
- API 자동 탐지 성공률 ≥ 80%
- 알림 전송 성공률 ≥ 95%

---

### Sprint 5 (Week 8-9): Cloud Integration
**Goal**: Obsidian 및 Dropbox 연동

**Deliverables**:
- [ ] Obsidian CLI 통합
- [ ] Dropbox Sync (Optional)
- [ ] 통합 검색 동작

**Tasks**:
- Epic 3.1, 3.2, 3.3
- Total Story Points: 32

**Sprint Review Criteria**:
- Obsidian 노트 생성
- Dropbox 동기화
- 통합 검색 정확도 ≥ 85%

---

### Sprint 6 (Week 10-11): Production
**Goal**: 프로덕션 준비 완료

**Deliverables**:
- [ ] 모든 테스트 통과
- [ ] 문서화 완료
- [ ] 배포 패키지 준비

**Tasks**:
- Epic 4.1, 4.2, 4.3, 4.4
- Total Story Points: 40

**Sprint Review Criteria**:
- LongMemEval ≥ 85%
- CI/CD 동작
- 문서 완성
- 설치 스크립트 동작

---

## 7. Resource Allocation

### 7.1 Required Technology Stack

#### Core Dependencies
```
Python 3.8+
├── openai >= 1.0.0           # LLM API
├── tiktoken >= 0.5.0         # Token counting
├── chromadb >= 0.4.0         # Vector store
├── pyyaml >= 6.0             # Config parsing
├── python-dotenv >= 1.0.0    # Environment variables
├── questionary >= 2.0.0      # Interactive CLI prompts
├── tenacity >= 8.0.0         # Retry policy
├── psutil >= 5.0.0           # Process scanning
└── watchdog >= 3.0.0         # File monitoring (optional)
```

#### Optional Dependencies
```
obsidian-cli                  # Obsidian integration
dropbox >= 11.0.0             # Dropbox API
pytest >= 7.0.0               # Testing
black >= 23.0.0               # Code formatting
mypy >= 1.0.0                 # Type checking
```

### 7.2 Development Environment

#### macOS
```bash
# Python 환경
brew install python@3.11

# 가상 환경
python3 -m venv venv
source venv/bin/activate

# 의존성 설치
pip install -r requirements.txt

# Obsidian CLI (Optional)
brew tap yakitrak/yakitrak
brew install yakitrak/yakitrak/obsidian-cli
```

#### Linux
```bash
# Python 환경
sudo apt install python3.11 python3.11-venv

# 가상 환경
python3 -m venv venv
source venv/bin/activate

# 의존성 설치
pip install -r requirements.txt
```

### 7.3 External Dependencies

| Dependency | Purpose | Required | Notes |
|------------|---------|----------|-------|
| **LLM API** | Observer/Reflector | Yes | Google Gemini or OpenAI |
| **ChromaDB** | Vector storage | Yes | Local persistent storage |
| **Obsidian** | Cold storage | No | Optional cloud backup |
| **Dropbox** | Cloud sync | No | Optional |

---

## 8. Quality Assurance

### 8.1 Test Plan

#### Unit Tests
```
tests/
├── test_watcher.py          # LogWatcher tests
├── test_observer.py         # Observer tests
├── test_reflector.py        # Reflector tests
├── test_merger.py           # MemoryMerger tests
├── test_memory_store.py     # ChromaDB tests
└── test_ttl_manager.py      # TTL tests
```

**Coverage Target**: ≥ 80%

#### Integration Tests
```
tests/
└── test_integration.py
    ├── test_end_to_end_pipeline
    ├── test_openclaw_integration
    └── test_obsidian_integration
```

#### Performance Tests
```
tests/
└── test_performance.py
    ├── test_observer_latency
    ├── test_search_latency
    └── test_memory_load
```

### 8.2 Code Review Process

**Pull Request Checklist**:
- [ ] 모든 테스트 통과
- [ ] Coverage ≥ 80%
- [ ] Type hints 추가
- [ ] Docstring 작성
- [ ] Linting 통과 (black, mypy)
- [ ] 리뷰어 승인

**Reviewers**:
- Lead Developer: 아르고
- QA: 아신 (예정)

### 8.3 CI/CD Pipeline

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - run: pip install -r requirements.txt
      - run: pytest tests/ --cov --cov-report=xml
      - uses: codecov/codecov-action@v3
```

---

## 9. Risk Mitigation

### 9.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **LLM API 장애** | Medium | High | 다중 모델 지원 (Google + OpenAI) |
| **토큰 초과** | Low | High | 자동 압축 강화, 토큰 모니터링 |
| **데이터 손실** | Low | High | 다중 백업 (Hot/Warm/Cold), 상태 파일 복원 |
| **ChromaDB 성능 저하** | Medium | Medium | 인덱스 최적화, 배치 처리 |
| **Obsidian 호환성** | Medium | Low | 직접 파일 조작 대안 구현 |
| **OpenClaw 업데이트** | Medium | Medium | Zero-Code-Modification 원칙 준수 |

### 9.2 Mitigation Plans

#### LLM API 장애 대응
```python
# Fallback mechanism
MODELS = [
    "google/gemini-2.5-flash",
    "openai/gpt-4o-mini",
    "anthropic/claude-3-haiku"
]

def observe_with_fallback(messages):
    for model in MODELS:
        try:
            return observer.observe(messages, model=model)
        except Exception as e:
            logger.warning(f"Model {model} failed: {e}")
            continue
    raise Exception("All models failed")
```

#### 토큰 초과 방지
```python
# Token budget monitoring
class TokenBudget:
    MAX_ACTIVE_MEMORY_TOKENS = 30_000
    MAX_OBSERVATION_BATCH = 10_000

    def check_and_compress(self, content):
        tokens = count_tokens(content)
        if tokens > self.MAX_ACTIVE_MEMORY_TOKENS:
            return reflector.reflect(content, level=2)
        return content
```

#### 데이터 손실 방지
```python
# Multi-layer backup
class BackupManager:
    def backup(self, observation):
        # Layer 1: ChromaDB
        memory_store.add_observation(observation)

        # Layer 2: Markdown file
        append_to_archive(observation)

        # Layer 3: State file
        save_state(observation.id)
```

---

## 10. Definition of Done

### 10.1 Task Level

각 Task는 다음 조건을 모두 충족해야 완료:
- [ ] 코드 작성 완료
- [ ] 단위 테스트 통과
- [ ] Coverage ≥ 80%
- [ ] Type hints 추가
- [ ] Docstring 작성
- [ ] Linting 통과 (black, mypy)
- [ ] Code Review 승인

### 10.2 User Story Level

각 User Story는 다음 조건을 모두 충족해야 완료:
- [ ] 모든 Task 완료
- [ ] Acceptance Criteria 충족
- [ ] 통합 테스트 통과
- [ ] 문서 업데이트
- [ ] Demo 가능

### 10.3 Epic Level

각 Epic은 다음 조건을 모두 충족해야 완료:
- [ ] 모든 User Story 완료
- [ ] Epic Goal 달성
- [ ] End-to-end 테스트 통과
- [ ] 성능 목표 달성
- [ ] Stakeholder 승인

### 10.4 Phase Level

각 Phase는 다음 조건을 모두 충족해야 완료:
- [ ] 모든 Epic 완료
- [ ] Phase Deliverables 제공
- [ ] 벤치마크 통과
- [ ] 문서 완성
- [ ] Production-ready

### 10.5 Project Level

프로젝트는 다음 조건을 모두 충족해야 완료:
- [ ] 모든 Phase 완료
- [ ] LongMemEval ≥ 85%
- [ ] 토큰 절약률 ≥ 90%
- [ ] 응답 지연 ≤ 1초
- [ ] 24시간 안정 동작
- [ ] 문서 완성
- [ ] 사용자 승인

---

## 11. Success Metrics

### 11.1 Key Performance Indicators (KPI)

| KPI | Target | Measurement | Priority |
|-----|--------|-------------|----------|
| **토큰 절약률** | ≥ 90% | (raw_tokens - compressed_tokens) / raw_tokens | P0 |
| **검색 정확도** | ≥ 85% | LongMemEval benchmark | P0 |
| **응답 지연** | ≤ 1초 | Memory load time | P0 |
| **데이터 손실** | 0% | Archive success rate | P0 |
| **압축률** | ≥ 5x | raw_tokens / compressed_tokens | P1 |
| **시스템 안정성** | ≥ 99.9% | Uptime percentage | P1 |

### 11.2 Acceptance Criteria

**Phase 1 (MVP)**:
- [ ] OpenClaw가 메모리 파일 읽기 성공
- [ ] 대화 맥락 유지 확인
- [ ] 토큰 절약률 ≥ 70%

**Phase 2 (Enhanced)**:
- [ ] Semantic Search 정확도 ≥ 85%
- [ ] 토큰 절약률 ≥ 90%
- [ ] TTL 정책 동작

**Phase 3 (Obsidian)**:
- [ ] Obsidian 노트 생성 성공
- [ ] 통합 검색 동작
- [ ] Dropbox 동기화 (Optional)

**Phase 4 (Production)**:
- [ ] LongMemEval ≥ 85%
- [ ] 24시간 안정 동작
- [ ] 문서 완성
- [ ] 설치 스크립트 동작

---

## 12. Timeline & Milestones

### 12.1 Gantt Chart

```
Week    1    2    3    4    5    6    7    8    9   10   11
       │    │    │    │    │    │    │    │    │    │    │
Phase 1 ████████████████████
       │ S1 │ S2 │         │    │    │    │    │    │    │
       │    │    │         │    │    │    │    │    │    │
Phase 2 │    │    │████████████████████
       │    │    │ S3 │ S4 │    │    │    │    │    │
       │    │    │    │    │    │    │    │    │    │
Phase 3 │    │    │    │    │████████████████
       │    │    │    │    │ S5 │    │    │    │    │
       │    │    │    │    │    │    │    │    │    │
Phase 4 │    │    │    │    │    │    │████████████
       │    │    │    │    │    │    │ S6 │    │
       │    │    │    │    │    │    │    │    │
Milestones:
M1: MVP Complete (Week 4)
M2: Enhanced Features (Week 7)
M3: Obsidian Integration (Week 9)
M4: Production Release (Week 11)
```

### 12.2 Critical Path

```
LogWatcher → Observer → MemoryMerger → OpenClaw Integration
                ↓
           ChromaDB → Reflector → TTL Manager
                                      ↓
                                 Obsidian → Production
```

---

## 13. Appendix

### 13.1 Story Point Scale

| Points | Complexity | Time | Example |
|--------|------------|------|---------|
| 1 | Trivial | 1-2h | Config 파일 작성 |
| 2 | Simple | 2-4h | 단순 클래스 구현 |
| 3 | Medium | 4-8h | 복잡한 로직 구현 |
| 5 | Complex | 1-2d | LLM 통합, 복잡한 알고리즘 |
| 8 | Very Complex | 2-3d | 아키텍처 설계 |

### 13.2 Priority Levels

| Priority | Description | SLA |
|----------|-------------|-----|
| **P0** | Critical | Must have for MVP |
| **P1** | Important | Should have for Enhanced |
| **P2** | Nice-to-have | Optional |

### 13.3 Glossary

| Term | Definition |
|------|------------|
| **Observation** | 로그에서 추출한 구조화된 정보 |
| **Reflection** | Observation을 압축한 장기 기억 |
| **Hot Memory** | ChromaDB에 저장된 최근 90일 메모리 |
| **Warm Memory** | Markdown 파일로 아카이브된 90일~1년 메모리 |
| **Cold Memory** | Obsidian에 저장된 1년 이상 메모리 |
| **TTL** | Time-To-Live, 메모리 수명 주기 |
| **Sidecar** | 독립 프로세스로 실행되는 보조 시스템 |

---

## 14. Change Log

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-12 | 아르고 | Initial version |
| 1.1 | 2026-02-12 | 아르고 | Added Epic 1.3: Setup Wizard (TUI) with 9 technical tasks (42 story points) |
| 1.2 | 2026-02-12 | 아르고 | Added Epic 2.4: Error Handling & Recovery to Phase 2 (45 story points) |

**Changes in v1.2**:
- Added **Epic 2.4: Error Handling & Recovery** to Phase 2
  - US-2.4.1: Retry Policy 구현 (12 points)
  - US-2.4.2: OpenClaw API 자동 탐지 (19 points)
  - US-2.4.3: HTTP API Hook 알림 (13 points)
  - US-2.4.4: Config 파일 스키마 확장 (7 points)
- Added 16 technical tasks (Task 2.4.1.1 ~ Task 2.4.4.3)
  - LLMRetryPolicy 클래스 구현
  - Observer/Reflector에 Retry 적용
  - 재시도 로깅 시스템
  - OpenClawAPIDetector 클래스
  - Config 파일 파서
  - 프로세스 스캔 (psutil)
  - 연결 테스트
  - TUI 통합 (Step 5: 에러 알림 설정)
  - ErrorNotifier 클래스 구현
  - HTTP API Hook 통합
  - config.yaml 스키마 확장
- Updated Phase 2 total story points: 58 → 103
- Updated Sprint 4 title: "TTL Management" → "TTL Management & Error Handling"
- Updated Sprint 4 story points: 16 → 61
- Added Phase 2 Summary section
- Added tenacity and psutil to dependencies

**Changes in v1.1**:
- Added **Epic 1.3: Setup Wizard (TUI)** to Phase 1
  - US-1.3.1: Setup wizard 기본 구조
  - US-1.3.2: 6단계 설정 플로우
  - US-1.3.3: Obsidian/Dropbox 선택 기능
  - US-1.3.4: API 키 안전 저장
- Added 9 technical tasks (Task 1.3.1.1 ~ Task 1.3.1.9)
  - questionary 통합
  - SetupWizard 클래스 구현
  - 6단계 메서드 구현
  - Obsidian/Dropbox 선택
  - config.yaml 생성 로직
  - .env 파일 관리
  - 유효성 검증
  - 에러 처리
  - 단위 테스트
- Updated Phase 1 total story points: 58 → 100
- Updated Sprint 1 deliverables to include Setup Wizard
- Renumbered Epic 1.3 → Epic 1.4 (Main Daemon Implementation)
- Added questionary to core dependencies
- Added Phase 1 Summary section

---

**문서 끝** 🎯

**Next Steps**:
1. Stakeholder 리뷰 및 승인
2. Sprint 1 시작
3. 개발 환경 설정
4. 첫 Task 착수 (Task 1.1.1.1)
