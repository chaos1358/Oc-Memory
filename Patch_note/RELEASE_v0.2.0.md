# Release v0.2.0 — LaunchAgent 안정화 + up/down 통합 명령어

**Release Date**: 2026-02-14
**OC-Guardian**: v0.2.0

---

## Overview

LaunchAgent의 `KeepAlive=true`로 인한 이중 감독 충돌과 중복 프로세스 문제를 해결했습니다.
OpenClaw의 자체 LaunchAgent와 guardian 사이의 역할을 분리하고, 전체 스택을 한 줄로 관리하는 `up`/`down` 명령어를 추가했습니다.

**핵심 변경**: OpenClaw을 `managed = false`(감시 전용)로 전환하여 Zero-Core-Modification 원칙을 준수하면서 충돌을 해소.

---

## Changes

### `oc-guardian up` / `down` 통합 명령어

- `oc-guardian up` — OpenClaw → OC-Memory 순서로 전체 스택을 한 줄로 시작
- `oc-guardian down` — OC-Memory → OpenClaw 역순으로 전체 스택 종료
- 기존 `start`/`stop`은 guardian 내부 managed 프로세스만 제어 (호환성 유지)

### `managed = false` 감시 전용 모드

- `ProcessConfig`에 `managed` 필드 추가 (기본값: `true`)
- `managed = false`인 프로세스는 guardian이 spawn/stop하지 않고 실행 상태만 감시
- sysinfo 기반 외부 프로세스 탐색 — `process_matches_with_args()`로 name, exe, cmd args 전체 매칭
- Node.js 래퍼(openclaw), venv Python 심링크(oc-memory) 등 복잡한 프로세스 구조 정확히 식별

### PID 파일 잠금

- guardian 시작 시 PID 파일(`/usr/local/etc/oc-guardian/guardian.pid`) 확인
- 기존 guardian이 살아있으면 시작 거부 → 중복 인스턴스 방지
- stale PID 파일(프로세스 없음) 자동 정리
- supervisor_loop 종료 시 PID 파일 삭제

### LaunchAgent 안정화

- `KeepAlive`를 조건부로 변경: `SuccessfulExit = false`
  - 정상 종료(exit 0) → 재시작하지 않음 (`oc-guardian stop` 가능)
  - 비정상 종료(crash) → 재시작 (안정성 유지)
- `ThrottleInterval = 30` 추가 — crash 시 30초 간격 재시작
- SIGTERM 핸들러 추가 — graceful shutdown으로 exit 0 보장

### 백업 스팸 방지

- `check_json_config_with_backup()`에서 기존 `.backup.1`과 내용 비교
- 내용이 동일하면 백업 생성을 건너뜀 → 불필요한 I/O 제거

---

## Changed Files

| File | Change |
|------|--------|
| `guardian/src/main.rs` | `up`/`down` 명령어, PID 파일 잠금, SIGTERM 핸들러 |
| `guardian/src/process.rs` | `managed=false` 분기 처리, `process_matches_with_args()` |
| `guardian/src/config.rs` | `managed` 필드 추가 |
| `guardian/src/health.rs` | 백업 중복 방지 |
| `guardian/Cargo.toml` | `libc` 의존성 추가 |
| `guardian/service/com.openclaw.guardian.plist` | KeepAlive 조건부, ThrottleInterval |
| `guardian.toml` | openclaw에 `managed = false` |
| `README.md` | up/down 명령어, Run in Background 섹션 갱신 |
| `CHANGELOG.md` | 신규 생성 |

---

## Architecture Change

```
Before (v1.x):
  guardian ──spawn──→ openclaw    ← launchd도 관리 (충돌!)
  guardian ──spawn──→ oc-memory

After (v0.2.0):
  launchd  ──관리──→ openclaw    ← 단일 관리자
  guardian ──감시──→ openclaw    ← 상태 확인만
  guardian ──관리──→ oc-memory   ← 시작/감시/재시작/종료
```

---

## Usage

```bash
# 전체 스택 시작 (OpenClaw → OC-Memory)
oc-guardian up

# 전체 스택 종료 (OC-Memory → OpenClaw)
oc-guardian down

# 상태 확인
oc-guardian status

# 백그라운드 실행 (LaunchAgent)
launchctl kickstart -k gui/$(id -u)/com.openclaw.guardian
```

---

## What's Next

- guardian이 OpenClaw 크래시를 감지했을 때 launchd에 재시작 요청하는 연동
- `oc-guardian up` 시 LaunchAgent 자동 등록 옵션
