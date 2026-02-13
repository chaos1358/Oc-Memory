# Release v1.0.6 — 덮개 닫기 슬립 방지: caffeinate → pmset 전환

**Release Date**: 2026-02-14
**OC-Memory**: v0.4.1 | **OC-Guardian**: v1.0.6

---

## Overview

MacBook 덮개를 닫았을 때 터미널 세션이 종료되는 문제의 근본 원인을 수정했습니다.

**핵심 발견**: `caffeinate -dis`는 덮개 닫기(lid-close) 슬립을 막지 **못합니다**.
Apple 공식 문서에 따르면 IOKit assertion(`PreventUserIdleSystemSleep`)은 lid-close를 명시적으로 제외합니다:

> "The system may still sleep for **lid close**, Apple menu, low battery, or other sleep reasons."

유일한 해결 방법은 `sudo pmset -a disablesleep 1`입니다.

---

## Changes

### OC-Guardian — v1.0.6

#### Bug Fixes
- **슬립 방지 방식을 caffeinate에서 pmset으로 전환**
  - `caffeinate -dis` → `sudo pmset -a disablesleep 1`
  - caffeinate는 유휴(idle) 슬립만 방지 가능, 덮개 닫기는 하드웨어 레벨 이벤트로 IOKit assertion 우회
  - pmset `disablesleep`은 커널 레벨에서 모든 슬립을 비활성화 (덮개 닫기 포함)

- **pmset 플래그 `-c` → `-a` 변경**
  - 기존: `-c` (charger/AC 전원만)
  - 변경: `-a` (all, 배터리+AC 모든 전원)

#### Configuration
- `use_caffeinate` 기본값: `true` → `false`
- `prevent_sleep` 기본값: `false` → `true` (guardian.toml)
- caffeinate 사용 시 경고 로그 출력 추가

---

## Migration Guide

```bash
# 1. 즉시 적용 (수동)
sudo pmset -a disablesleep 1

# 2. Guardian이 자동으로 pmset 실행하도록 sudoers 설정
sudo visudo -f /etc/sudoers.d/oc-guardian
# 내용: ailkisap ALL=(ALL) NOPASSWD: /usr/bin/pmset

# 3. Guardian 재시작
launchctl unload ~/Library/LaunchAgents/com.openclaw.guardian.plist
launchctl load ~/Library/LaunchAgents/com.openclaw.guardian.plist

# 4. 슬립 방지 확인
pmset -g | grep disablesleep
```

---

## Technical Details

| 방법 | 유휴 슬립 | 덮개 닫기 | sudo 필요 | 배터리 |
|------|:---------:|:---------:|:---------:|:------:|
| `caffeinate -dis` | O | **X** | X | `-s` AC만 |
| `pmset -a disablesleep 1` | O | **O** | O | O |

---

## What's Next
- sudoers NOPASSWD 자동 설정 스크립트 추가
- 발열 모니터링 (덮개 닫고 사용 시 온도 감시)
