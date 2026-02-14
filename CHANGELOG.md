# Changelog

## v0.2.0 — LaunchAgent 안정화 + 중복 프로세스 방지

### 문제

- `KeepAlive=true` 설정으로 인해 `oc-guardian stop` 후 launchd가 즉시 새 인스턴스를 생성
- guardian 재시작 시 기존 자식 프로세스(oc-memory)가 종료되지 않아 중복 실행
- OpenClaw의 `ai.openclaw.gateway.plist`와 guardian의 openclaw 프로세스 관리가 충돌
- 매 health check마다 config 백업이 생성되어 불필요한 I/O 발생

### 변경 사항

#### `managed = false` 지원 (config.rs, process.rs)

- `ProcessConfig`에 `managed` 필드 추가 (기본값: `true`)
- `managed = false`인 프로세스는 감시 전용 — guardian이 spawn/stop하지 않음
- 외부 프로세스를 sysinfo로 탐색하여 PID만 기록하고 health check 수행
- `start_all()`에서 외부 프로세스가 나타날 때까지 폴링 대기
- `stop_process()`에서 외부 프로세스는 중지하지 않고 추적 상태만 해제
- guardian.toml의 `[processes.openclaw]`에 `managed = false` 적용

#### PID 파일 잠금 (main.rs)

- `handle_start()`: 시작 전 PID 파일 확인 — 기존 guardian이 살아있으면 시작 거부
- stale PID 파일(프로세스 없음) 자동 정리
- supervisor_loop 종료 시 PID 파일 삭제
- `handle_stop()`: PID 파일의 guardian PID에 SIGTERM 전송 후 managed 프로세스만 정리

#### 백업 스팸 방지 (health.rs)

- `check_json_config_with_backup()`에서 백업 생성 전 기존 `.backup.1`과 내용 비교
- 내용이 동일하면 백업 생성을 건너뜀

#### `up` / `down` 통합 명령어 (main.rs)

- `oc-guardian up`: 외부 프로세스(OpenClaw) 먼저 시작 → guardian 시작 → managed 프로세스(OC-Memory) 시작
- `oc-guardian down`: managed 프로세스 중지 → 외부 프로세스 중지 — 전체 스택을 한 줄로 관리
- 프로세스 매칭 강화: `process_matches_with_args()` — name, exe, cmd args 전체를 검사하여 Node.js(openclaw), venv Python(oc-memory) 정확히 식별

#### LaunchAgent plist 변경

- `KeepAlive`를 조건부로 변경: `SuccessfulExit = false`
  - 정상 종료(exit 0) → 재시작하지 않음 (oc-guardian stop 가능)
  - 비정상 종료(crash) → 재시작 (안정성 유지)
- `ThrottleInterval = 30` 추가 — 30초 미만 재시작 방지

### 수정 파일

| 파일 | 변경 |
|------|------|
| `guardian/src/config.rs` | `managed` 필드 추가 |
| `guardian/src/process.rs` | managed=false 분기 처리, 외부 프로세스 탐색 |
| `guardian/src/main.rs` | PID 파일 잠금, handle_stop 개선 |
| `guardian/src/health.rs` | 백업 중복 방지 |
| `guardian/Cargo.toml` | `libc` 의존성 추가 |
| `guardian/service/com.openclaw.guardian.plist` | KeepAlive 조건부, ThrottleInterval |
| `guardian/guardian.toml` | openclaw에 managed=false |
| `guardian.toml` | openclaw에 managed=false |
