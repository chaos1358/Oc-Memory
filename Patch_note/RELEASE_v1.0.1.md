# Release v1.0.1 — Hotfix

**Release Date**: 2026-02-14
**OC-Memory**: v0.4.0 | **OC-Guardian**: v1.0.1

---

## Overview

이번 릴리즈는 macOS 환경에서 OC-Guardian 빌드 시 발생하던 치명적인 오류를 수정한 긴급 패치입니다.

---

## Changes

### 🛡️ OC-Guardian (Rust) — v1.0.1

#### Bug Fixes
- **macOS Build Error** (`guardian/src/macos.rs`)
  - **Issue**: `warn!` 매크로를 사용함에도 불구하고 `tracing::warn` 임포트가 누락되어 macOS 빌드가 실패하던 현상 수정.
  - **Fix**: `use tracing::{info, warn};`로 임포트 구문을 수정하여 빌드 정상화 완료.

#### Chores
- **.gitignore 업데이트**
  - 빌드 결과물인 `oc-guardian` 바이너리 파일이 깃 추적 대상에서 제외되도록 설정했습니다.

---

## How to Apply

```bash
git pull origin main
cd guardian
cargo build --release
cp target/release/oc-guardian ../oc-guardian
```

---

## What's Next
- WebSocket 실시간 메모리 주입 기능 개발 착수
- 웹 대시보드 프로토타입 설계
