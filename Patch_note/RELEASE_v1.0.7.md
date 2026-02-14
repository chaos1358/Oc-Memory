# Release v1.0.7 — 이메일 알림 발송 실패 수정

**Release Date**: 2026-02-14
**OC-Memory**: v0.4.1 | **OC-Guardian**: v1.0.7

---

## Overview

`kill -9`로 oc-memory를 강제 종료했을 때 guardian이 크래시를 감지하고 자동 재시작은 하지만 이메일 알림이 발송되지 않는 문제를 수정했습니다. 3가지 원인을 동시에 해결합니다.

---

## Changes

### OC-Guardian — v1.0.7

#### Bug Fixes

- **이벤트 이름 불일치 수정 (guardian.toml)**
  - 기존: `events = ["crash", "give_up", "config_error", "memory_leak"]`
  - 수정: `events = ["process_crash", "health_check_failed", "guardian_startup", "guardian_shutdown"]`
  - notification.rs의 `EventType::to_string()`이 `"process_crash"`을 반환하는데, config에는 `"crash"`로 되어 있어 필터링에서 전부 스킵됨

- **python → python3 fallback (notification.rs)**
  - macOS에는 `python` 명령어가 없고 `python3`만 존재
  - `python3`를 우선 실행하고, 실패 시 `python`으로 fallback

- **SMTP_PASSWORD 환경변수 전달 (LaunchAgent plist)**
  - LaunchAgent에서 실행되는 프로세스는 셸 환경변수를 상속받지 못함
  - plist의 `EnvironmentVariables`에 `SMTP_PASSWORD` 추가
  - plist 파일을 `.gitignore`에 추가하여 비밀번호 유출 방지

---

## Root Cause Analysis

| 원인 | 증상 | 수정 |
|------|------|------|
| 이벤트 이름 불일치 | 모든 알림이 필터에 걸려 스킵 | toml 이벤트명을 코드와 일치 |
| `python` 명령어 없음 | 이메일 전송 스크립트 실행 실패 | python3 우선 + python fallback |
| SMTP_PASSWORD 미설정 | Gmail SMTP 인증 실패 | plist에 환경변수 추가 |

---

## Verification

```bash
# 1. Guardian 재시작
oc-guardian stop && oc-guardian up

# 2. oc-memory 강제 종료
kill -9 $(pgrep -f memory_observer)

# 3. 로그 확인
tail -f /usr/local/var/log/oc-guardian/stdout.log
# 기대 출력:
# "Notification sent for event 'health_check_failed' on process 'oc-memory'"
# "Recovery scenario 'process_crash' matched for process 'oc-memory'"

# 4. 이메일 수신 확인
```

---

## Files Changed

| 파일 | 변경 |
|------|------|
| `guardian/src/notification.rs` | python3 우선 실행 + python fallback |
| `guardian.toml` | 이벤트 이름 수정 |
| `guardian/service/com.openclaw.guardian.plist` | SMTP_PASSWORD 환경변수 추가 |
| `.gitignore` | plist 파일 추가 (비밀번호 유출 방지) |
