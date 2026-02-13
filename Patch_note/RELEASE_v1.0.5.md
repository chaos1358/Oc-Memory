# Release v1.0.5 — LaunchAgent 서비스 등록 버그 수정

**Release Date**: 2026-02-14
**OC-Memory**: v0.4.1 | **OC-Guardian**: v1.0.5

---

## Overview

macOS LaunchAgent(plist)로 OC-Guardian을 등록할 때 CLI 인수 순서 오류로 서비스가 시작되지 않던 문제를 수정했습니다.

---

## Changes

### 🛡️ OC-Guardian — v1.0.5

#### Bug Fixes
- **LaunchAgent plist 인수 순서 수정** (`guardian/service/com.openclaw.guardian.plist`)
  - **Issue**: `oc-guardian start --config guardian.toml` 순서로 인수가 전달되어 `unexpected argument '--config'` 에러로 서비스 시작 실패
  - **Fix**: `oc-guardian --config guardian.toml start`로 글로벌 옵션이 서브커맨드 앞에 오도록 수정
  - `--config`는 clap의 글로벌 옵션이므로 서브커맨드(`start`) 앞에 위치해야 함

---

## How to Apply

```bash
# 최신 코드 반영
git pull origin main

# LaunchAgent 재등록
cd guardian/service
./install-service.sh install
```

---

## What's Next
- LaunchAgent 설치 스크립트 개선
- 서비스 상태 모니터링 명령어 추가
