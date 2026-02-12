# OC-Memory-Sidecar: 제품 요구사항 문서 (PRD)
## OpenClaw 관찰 메모리 시스템 (Observational Memory System)

**버전**: 2.0
**최종 업데이트**: 2026-02-12
**작성자**: Argo (OpenClaw 총괄 매니저)
**상태**: 초안
**문서 유형**: 제품 요구사항 문서

---

## 목차 (Table of Contents)

1. [제품 개요](#1-product-overview)
2. [사용자 페르소나 및 사용 사례](#2-user-personas--use-cases)
3. [기능 요구사항](#3-feature-requirements)
4. [사용자 스토리](#4-user-stories)
5. [기능 요구사항 상세](#5-functional-requirements)
6. [비기능 요구사항](#6-non-functional-requirements)
7. [데이터 모델](#7-data-model)
8. [사용자 인터페이스 (CLI)](#8-user-interface-cli)
9. [통합 요구사항](#9-integration-requirements)
10. [승인 기준](#10-acceptance-criteria)
11. [마일스톤 및 타임라인](#11-milestones--timeline)

---

## 1. Product Overview (제품 개요)

### 1.1 Product Name (제품명)

**OC-Memory-Sidecar** (OpenClaw 관찰 메모리 사이드카)

### 1.2 Vision Statement (비전 선언문)

OpenClaw 에이전트가 핵심 코드베이스를 수정하지 않고 장기 메모리 기능을 유지할 수 있도록 하며, 원활하고 토큰 효율적이며 프라이버시 우선 메모리 시스템을 제공하여 지능형 대화 컨텍스트 유지를 통해 사용자 경험을 향상시킵니다.

### 1.3 Product Positioning (제품 포지셔닝)

OC-Memory-Sidecar는 OpenClaw의 "사이드카 (Sidecar)"로 설계된 독립형 메모리 관리 시스템으로, Mastra 아키텍처에서 영감을 받은 관찰 메모리 (Observational Memory) 패턴을 구현하면서 핵심 OpenClaw 시스템과 완전히 독립적으로 유지됩니다.

### 1.4 Core Value Propositions (핵심 가치 제안)

| 가치 제안 | 설명 | 목표 지표 |
|-----------|------|-----------|
| **핵심 수정 불필요** | OpenClaw 코드베이스 변경 불필요 | 0건의 핵심 수정 |
| **업데이트 안전성** | OpenClaw 버전 업데이트에도 안전 | 100% 호환성 |
| **토큰 효율성** | 컨텍스트 사용량 대폭 감소 | 79-99% 토큰 절감 |
| **프라이버시 우선** | 모든 데이터 로컬 저장 | 클라우드 전송 0건 |
| **지능성** | 시맨틱 검색 및 컨텍스트 이해 | 90%+ 검색 정확도 |

### 1.5 Market Context (시장 상황) (2026)

**OpenClaw 최신 업데이트 (v2026.2.6)**:
- Claude Opus 4.6 및 GPT-5.3-Codex 지원
- Voyage AI 네이티브 메모리 지원
- 스킬 및 플러그인용 향상된 보안 스캐너
- 웹 UI의 토큰 사용량 대시보드
- 세션 히스토리 페이로드 제한

**Mastra 관찰 메모리 (Observational Memory) (2026)**:
- GPT-5-mini로 94.87% LongMemEval 점수 달성
- 비동기 버퍼링 기능 (2026년 2월)
- 5-40배 압축 비율
- 인라인 관찰 마커가 있는 OM 인식 UI

### 1.6 Problem Statement (문제 정의)

OpenClaw를 포함한 현재 AI 에이전트는 심각한 메모리 제한을 겪고 있습니다:

| 문제 | 영향 | 사용자 불편 |
|------|------|-------------|
| **컨텍스트 윈도우 제한** | 토큰 제한 후 이전 대화 망각 | 사용자가 정보를 반복해야 함 |
| **토큰 낭비** | 매번 전체 대화 히스토리 로드 | 비용 및 지연 증가 |
| **장기 학습 부재** | 세션 간 사용자 선호도 유지 불가 | 개인화 부족 |
| **세션 격리** | 각 대화가 제로에서 시작 | 워크플로우 연속성 부재 |

### 1.7 Solution Overview (솔루션 개요)

OC-Memory-Sidecar는 3계층 계층적 메모리 시스템을 구현합니다:

```
┌─────────────────────────────────────────────────────────────┐
│                   3계층 메모리 아키텍처                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  🔥 핫 메모리 (Hot Memory) (0-90일)                         │
│     → ChromaDB + active_memory.md                           │
│     → 실시간 시맨틱 검색                                     │
│     → ~30MB 저장공간                                         │
│                                                              │
│  ♨️  웜 메모리 (Warm Memory) (90-365일)                     │
│     → 마크다운 아카이브 파일                                 │
│     → 온디맨드 검색                                          │
│     → ~85MB 저장공간                                         │
│                                                              │
│  ❄️  콜드 메모리 (Cold Memory) (365일 이상)                 │
│     → Obsidian 볼트 + 클라우드 동기화                        │
│     → 장기 보존                                              │
│     → 무제한 저장공간                                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. User Personas & Use Cases (사용자 페르소나 및 사용 사례)

### 2.1 Primary Persona: The Power Developer (주요 페르소나: 파워 개발자)

**프로필**:
- **이름**: Alex Chen
- **역할**: 시니어 풀스택 개발자
- **사용량**: 하루 50회 이상 OpenClaw 상호작용
- **워크플로우**: 다중 프로젝트, 컨텍스트 집약적 개발
- **불편 사항**:
  - 프로젝트 컨텍스트를 지속적으로 재설명
  - 이전 결정 추적 실패
  - 반복적인 설명에 시간 낭비

**목표**:
- OpenClaw가 프로젝트 제약사항 기억
- 이전 결정과 이유 보존
- 세션 간 원활한 연속성
- 최소한의 수동 개입

### 2.2 Secondary Persona: The Research Analyst (보조 페르소나: 연구 분석가)

**프로필**:
- **이름**: Dr. Sarah Martinez
- **역할**: AI 연구 분석가
- **사용량**: 하루 30회 이상 OpenClaw 세션
- **워크플로우**: 문헌 검토, 데이터 분석, 보고서 작성
- **불편 사항**:
  - 이전 연구 결과 참조 불가
  - 논문을 반복적으로 재요약해야 함
  - 이전 세션의 인사이트 손실

**목표**:
- 장기 지식 유지
- 모든 상호작용에 대한 시맨틱 검색
- 연구 타임라인 추적
- 시간 경과에 따른 인사이트 집계

### 2.3 Tertiary Persona: The Hobbyist Creator (3차 페르소나: 취미 크리에이터)

**프로필**:
- **이름**: Jamie Lin
- **역할**: 콘텐츠 크리에이터 및 작가
- **사용량**: 주당 10-15회 OpenClaw 세션
- **워크플로우**: 스토리 작성, 브레인스토밍, 창작 프로젝트
- **불편 사항**:
  - 세션 간 캐릭터 세부사항 망각
  - 시간 경과에 따른 플롯 포인트 손실
  - 일관성 없는 창작 방향

**목표**:
- 캐릭터 및 플롯 추적
- 창작 연속성
- 스타일 선호도 유지
- 저유지보수 시스템

### 2.4 Use Case Scenarios (사용 사례 시나리오)

#### Use Case 1: Multi-Day Project Development (다일 프로젝트 개발)

**시나리오**: Alex가 2주 동안 마이크로서비스 아키텍처를 구축합니다.

**OC-Memory 없이**:
```
1일차: "Node.js와 PostgreSQL로 마이크로서비스 앱을 만들자..."
5일차: 사용자: "어떤 데이터베이스를 선택했지?"
       OpenClaw: "그 정보가 없습니다..."
10일차: 사용자가 전체 프로젝트 아키텍처를 재설명해야 함
```

**OC-Memory 사용**:
```
1일차: "Node.js와 PostgreSQL로 마이크로서비스 앱을 만들자..."
       [관찰 저장: 기술 스택 결정]
5일차: 사용자: "어떤 데이터베이스를 선택했지?"
       OpenClaw: "마이크로서비스 프로젝트를 위해 PostgreSQL을 선택했습니다."
10일차: OpenClaw가 자동으로 이전 아키텍처 결정 참조
```

**제공 가치**: 컨텍스트 재설명 시간 90% 감소

#### Use Case 2: Research Literature Review (연구 문헌 검토)

**시나리오**: Dr. Martinez가 3개월 동안 50편의 논문을 검토합니다.

**OC-Memory 없이**:
```
1개월차: 논문 A-D 검토, 메모 작성
2개월차: 논문 E-H 검토, A-D의 인사이트 망각
3개월차: 특정 발견이 논의된 위치를 찾을 수 없음
```

**OC-Memory 사용**:
```
1개월차: 논문 A-D 검토, 관찰 저장
2개월차: 논문 E-H 검토, "X에 대한 발견" 쿼리 가능
3개월차: 시맨틱 검색으로 모든 논문의 관련 논의 검색
```

**제공 가치**: 정보 검색 85% 더 빠름

#### Use Case 3: Creative Writing Continuity (창작 작문 연속성)

**시나리오**: Jamie가 복잡한 캐릭터 관계가 있는 소설을 씁니다.

**OC-Memory 없이**:
```
1주차: 파란 눈을 가진 캐릭터 A 소개
5주차: 실수로 캐릭터 A를 녹색 눈으로 작성
결과: 연속성 오류
```

**OC-Memory 사용**:
```
1주차: 캐릭터 세부사항이 관찰에 저장됨
5주차: OpenClaw가 상기: "캐릭터 A는 파란 눈으로 묘사되었습니다"
결과: 일관된 캐릭터 묘사
```

**제공 가치**: 연속성 오류 95% 감소

#### Use Case 4: Learning and Skill Development (학습 및 스킬 개발)

**시나리오**: 사용자가 OpenClaw로 새로운 프로그래밍 언어를 학습합니다.

**OC-Memory 없이**:
```
세션 1: "JavaScript의 async/await에 어려움을 겪습니다"
세션 10: OpenClaw가 async/await을 다시 처음부터 설명
결과: 비효율적인 학습
```

**OC-Memory 사용**:
```
세션 1: 사용자 어려움이 관찰에 기록됨
세션 10: OpenClaw가 이전 설명을 참조하고 그 위에 구축
결과: 점진적이고 개인화된 학습 경로
```

**제공 가치**: 학습 진행 70% 더 빠름

---

## 3. Feature Requirements (기능 요구사항)

### 3.1 P0 Features (Must Have) (P0 기능 - 필수)

MVP 출시에 필수적인 기능입니다. 이 기능들이 없으면 제품이 작동할 수 없습니다.

#### FR-P0-001: Real-Time Event Detection (실시간 이벤트 감지)

**설명**: OpenClaw의 활동을 실시간으로 감지하여 메모리 업데이트를 트리거합니다.

**구현 옵션**:

**옵션 A (권장): Webhook Hook**
- OpenClaw Webhook API 사용 (`POST /hooks/agent`)
- 실시간 양방향 통신
- 파일 변경 시 즉시 알림 전송
- 안정적이고 이벤트 기반

**옵션 B: Session Transcript Watching**
- 경로: `~/.openclaw/agents/<agentId>/sessions/<sessionId>.jsonl`
- 포맷: JSONL (line-by-line JSON)
- 내용: 완전한 대화 히스토리 + Tool calls + Results
- 파일 감시 (tail-f 패턴 또는 chokidar)

**옵션 C: Command Log Watching (보조)**
- 경로: `~/.openclaw/logs/commands.log` (command-logger hook 활성화 필요)
- 포맷: JSONL
- 내용: `/new`, `/reset`, `/stop` 커맨드 기록

**요구사항**:
- 1초 미만의 지연으로 새 이벤트 감지
- 파일 변경 및 로테이션 처리
- 시스템 재시작 간 상태 유지
- Webhook 인증 (Bearer token)

**승인 기준**:
- ✅ 새 이벤트가 1초 이내에 감지됨
- ✅ Webhook 전송 성공률 >95%
- ✅ 재시작 간 상태 지속성
- ✅ 유휴 모니터링 중 CPU 사용량 <5%

#### FR-P0-002: Observation Generation (관찰 생성)

**설명**: LLM을 사용하여 원시 로그 데이터를 구조화된 관찰로 변환합니다.

**요구사항**:
- 관찰자 (Observer) 에이전트를 사용한 대화 로그 파싱
- 우선순위별 관찰 분류 (🔴 높음, 🟡 중간, 🟢 낮음)
- 유형별 분류 (상태, 결정, 선호도, 작업)
- 시간 정보 추출 (타임스탬프)
- 각 관찰에 대한 고유 ID 생성
- 효율성을 위한 배치 처리 지원

**승인 기준**:
- ✅ 로그 항목 후 5초 이내에 관찰 생성
- ✅ 우선순위 분류 정확도 >85%
- ✅ 모든 관찰에 타임스탬프 포함
- ✅ 고유 ID 충돌률 <0.001%

#### FR-P0-003: Active Memory File Management (활성 메모리 파일 관리)

**설명**: OpenClaw가 읽을 수 있는 최신 `active_memory.md` 파일을 유지합니다.

**요구사항**:
- 마크다운 형식의 메모리 파일 생성
- 파일을 4000 토큰 미만으로 유지 (구성 가능)
- 관찰이 누적됨에 따라 파일 자동 업데이트
- 쉬운 LLM 파싱을 위한 콘텐츠 구조화
- 메타데이터 포함 (마지막 업데이트, 토큰 수)
- 손상 방지를 위한 원자적 파일 쓰기

**승인 기준**:
- ✅ 파일 업데이트가 2초 이내에 완료
- ✅ 토큰 제한 절대 초과하지 않음
- ✅ 파일이 항상 유효한 마크다운 형식
- ✅ 동시 액세스 중 파일 손상 없음

#### FR-P0-004: OpenClaw Memory Integration (OpenClaw 메모리 통합)

**설명**: OpenClaw가 OC-Memory의 메모리 파일을 자동으로 인덱싱하고 검색할 수 있도록 합니다.

**구현 방법**:

**방법 A (권장): Memory Files Auto-Indexing**
- OC-Memory가 `~/.openclaw/workspace/memory/*.md` 에 Markdown 파일 작성
- OpenClaw가 자동으로 파일 감시 (chokidar) 및 인덱싱
- SQLite + Vector (sqlite-vec) + FTS5 자동 인덱싱
- 에이전트가 `memory_search` tool로 검색

**방법 B: openclaw.json 설정 (정적 프롬프트)**
```json
{
  "agents": {
    "main": {
      "systemPrompt": "You have access to OC-Memory system...",
      "contextFiles": ["~/oc-memory/active_memory.md"],
      "memory": {
        "enabled": true,
        "extraPaths": ["~/oc-memory/archive"]
      }
    }
  }
}
```

**방법 C: Plugin Hook (동적 프롬프트)**
- 위치: `~/.openclaw/plugins/oc-memory/index.js`
- Hook: `before_agent_start`
- 동적으로 메모리 컨텍스트 주입

**요구사항**:
- Zero-Core-Modification (OpenClaw 코드 수정 불필요)
- 명확한 통합 지침 제공
- 다중 OpenClaw 버전 지원
- 메모리 파일 자동 인덱싱 확인

**승인 기준**:
- ✅ OpenClaw v2026.2.x에서 통합 작동
- ✅ OpenClaw 핵심 코드 수정 없음
- ✅ Memory 파일 자동 인덱싱 확인
- ✅ `memory_search` tool 정상 동작
- ✅ 명확한 문서 제공

#### FR-P0-007: Interactive Setup Wizard (TUI) (대화형 설정 마법사 - TUI)

**설명**: 최초 구성을 위한 텍스트 기반 사용자 인터페이스 (TUI) 설정 마법사를 제공합니다.

**요구사항**:
- 대화형 6단계 설정 프로세스
- 기본 구성을 통한 사용자 안내
- 메모리 계층 설정 구성 (핫/웜/콜드 TTL)
- LLM 공급자 및 API 키 설정
- 선택적 기능 활성화 (Obsidian, Dropbox)
- config.yaml 및 .env 파일 자동 생성
- 저장 전 모든 입력 검증
- 중단된 설정 재개 지원

**승인 기준**:
- ✅ 설정 마법사가 5분 이내에 완료
- ✅ 모든 필수 구성 값 수집
- ✅ 저장 전 API 키 검증
- ✅ 올바른 형식으로 config.yaml 생성
- ✅ 적절한 권한(600)으로 .env 파일 생성
- ✅ 잘못된 입력에 대한 명확한 오류 메시지
- ✅ 사용자가 선택적 기능 건너뛸 수 있음
- ✅ 구성 업데이트를 위해 설정 재실행 가능

### 3.2 P1 Features (Should Have) (P1 기능 - 권장)

전체 기능에는 중요하지만 출시에 필수적이지 않은 기능입니다.

#### FR-P1-001: Memory Storage & Search (메모리 저장 및 검색)

**설명**: 메모리 데이터를 저장하고 시맨틱 검색을 제공합니다.

**구현 옵션**:

**옵션 A (권장): OpenClaw 내장 Memory 시스템 활용**
- 위치: `~/.openclaw/agents/<agentId>/memory.db` (SQLite)
- 벡터 검색: sqlite-vec extension (1536차원)
- Full-text 검색: FTS5
- 자동 인덱싱: `memory/*.md` 파일 감시
- 검색 도구: `memory_search`, `memory_get`
- 장점: 중복 제거, OpenClaw 기능 최대 활용, 단순함

**옵션 B (선택): 외부 ChromaDB (분석용)**
- 용도: OC-Memory 자체 분석, 통계, 패턴 인식
- 위치: `~/.oc-memory/chromadb/`
- 영구 모드에서 ChromaDB 사용
- 임베딩 모델: all-MiniLM-L6-v2 (384차원)
- 고급 시맨틱 검색 및 클러스터링

**요구사항**:
- 메타데이터 유지 (우선순위, 카테고리, 타임스탬프)
- 효율적인 인덱싱 구현
- 증분 업데이트 지원
- OpenClaw Memory와 동기화 (옵션 B 선택 시)

**승인 기준**:
- ✅ 검색 결과가 500ms 이내에 반환
- ✅ 테스트 쿼리에서 검색 정확도 >85%
- ✅ OpenClaw `memory_search` tool 정상 동작
- ✅ 재시작 중 데이터 손실 없음

#### FR-P1-002: Semantic Search API (시맨틱 검색 API)

**설명**: 모든 관찰에서 의미 기반 검색을 활성화합니다.

**요구사항**:
- 자연어 쿼리 지원
- 유사도 순위 지정
- 메타데이터 필터링 (날짜 범위, 우선순위, 카테고리)
- 결과 관련성 점수
- 상위 k개 결과 검색
- 쿼리 결과 캐싱

**승인 기준**:
- ✅ 쿼리 응답 시간 <500ms
- ✅ 관련성 정확도 >85%
- ✅ 복잡한 쿼리 지원 (다중 필터)
- ✅ 반복 쿼리에 대한 캐시 적중률 >60%

#### FR-P1-003: Time-To-Live (TTL) Management (TTL 관리)

**설명**: 연령에 따라 오래된 관찰을 자동으로 아카이브합니다.

**요구사항**:
- 구성 가능한 TTL 기간 (기본값: 핫의 경우 90일)
- 자동 핫 → 웜 전환
- 웜 → 콜드 전환에 대한 사용자 승인
- 예약된 백그라운드 작업 (일일)
- 아카이브 알림 시스템
- 롤백 기능

**승인 기준**:
- ✅ TTL 작업이 사용자 개입 없이 매일 실행
- ✅ 핫 → 웜 전환 100% 자동
- ✅ 웜 → 콜드는 명시적 사용자 승인 필요
- ✅ 아카이브 작업이 10분 이내에 완료

#### FR-P1-004: 3-Tier Memory Architecture (3계층 메모리 아키텍처)

**설명**: 핫, 웜, 콜드 계층으로 계층적 저장소를 구현합니다.

**요구사항**:
- **핫 메모리**: ChromaDB + active_memory.md (0-90일)
- **웜 메모리**: 마크다운 아카이브 (90-365일)
- **콜드 메모리**: Obsidian 볼트 (365일 이상)
- 계층 간 원활한 데이터 마이그레이션
- 계층 전반에 걸쳐 일관된 데이터 형식
- 계층별 액세스 패턴

**승인 기준**:
- ✅ 핫 메모리: 실시간 액세스 (<100ms)
- ✅ 웜 메모리: 온디맨드 액세스 (<2s)
- ✅ 콜드 메모리: 수동 액세스 (사용자 시작)
- ✅ 계층 전환 중 데이터 손실 없음

#### FR-P1-005: Observation Compression (관찰 압축)

**설명**: 반영자 (Reflector) 에이전트를 사용하여 관찰을 주기적으로 압축합니다.

**요구사항**:
- 관련 관찰 배치 압축
- 토큰을 줄이면서 주요 정보 보존
- 시간적 컨텍스트 유지
- 토큰 임계값 도달 시 압축 실행
- 수동 압축 트리거 지원
- 감사를 위해 원본 관찰 보존

**승인 기준**:
- ✅ 압축 비율 5:1 ~ 40:1
- ✅ 정보 유지 >90%
- ✅ 압축이 30초 이내에 완료
- ✅ 필요시 원본 데이터 복구 가능

#### FR-P1-006: CLI Interface (CLI 인터페이스)

**설명**: 시스템 관리 및 쿼리를 위한 명령줄 인터페이스입니다.

**요구사항**:
- 데몬 시작/중지
- 메모리 쿼리 (검색)
- 통계 보기 (토큰 사용량, 관찰 수)
- 수동 아카이브 작업
- 구성 관리
- 상태 모니터링

**승인 기준**:
- ✅ 모든 명령이 문서화됨
- ✅ 명령 응답 시간 <2s
- ✅ 오류 메시지가 명확하고 실행 가능함
- ✅ 도움말 시스템이 포괄적임

### 3.3 P2 Features (Nice to Have) (P2 기능 - 선택)

제품을 향상시키지만 향후 릴리스로 연기할 수 있는 기능입니다.

#### FR-P2-001: Obsidian Integration (Obsidian 통합)

**설명**: 콜드 메모리 저장소 및 시각화를 위해 Obsidian과 통합합니다.

**요구사항**:
- Obsidian CLI 통합 (Yakitrak)
- 자동 볼트 생성
- 마크다운 형식 호환성
- 그래프 뷰 지원
- 태그 기반 조직
- 양방향 동기화 지원

**승인 기준**:
- ✅ Obsidian 볼트에 아카이브 생성
- ✅ Obsidian에서 마크다운 파일 읽기 가능
- ✅ 태그가 올바르게 적용됨
- ✅ 그래프 뷰가 메모리 연결 표시

#### FR-P2-002: Dropbox Cloud Sync (Dropbox 클라우드 동기화)

**설명**: Dropbox를 통한 자동 클라우드 백업입니다.

**요구사항**:
- Dropbox API 통합
- 콜드 메모리 생성 후 자동 동기화
- 충돌 해결
- 대역폭 조절
- 선택적 동기화 (핫 메모리 제외)
- 오프라인 모드 지원

**승인 기준**:
- ✅ 동기화가 5분 이내에 완료
- ✅ 동기화 중 데이터 손실 없음
- ✅ 충돌이 자동으로 해결되거나 플래그됨
- ✅ 대기열 기반 동기화로 오프라인 작동

#### FR-P2-003: Reverse Query (Cloud → Local) (역 쿼리 - 클라우드 → 로컬)

**설명**: Dropbox 또는 Obsidian에서 직접 콜드 메모리를 쿼리합니다.

**요구사항**:
- Dropbox API 파일 검색
- Obsidian REST API 통합
- 원격 콘텐츠 검색
- 결과 캐싱
- 클라우드를 사용할 수 없는 경우 로컬로 폴백

**승인 기준**:
- ✅ 원격 쿼리가 5초 이내에 결과 반환
- ✅ 캐시가 반복 쿼리 시간 80% 단축
- ✅ 클라우드 오프라인 시 우아한 저하
- ✅ 결과가 로컬 쿼리와 동일

#### FR-P2-004: Web Dashboard (웹 대시보드)

**설명**: 메모리 관리 및 분석을 위한 시각적 인터페이스입니다.

**요구사항**:
- 메모리 사용량 시각화 (차트)
- 관찰 타임라인 뷰
- 검색 인터페이스
- TTL 상태 표시
- 구성 편집기
- 토큰 사용량 분석

**승인 기준**:
- ✅ 대시보드가 3초 이내에 로드
- ✅ 5초마다 실시간 업데이트
- ✅ 반응형 디자인 (모바일에서 작동)
- ✅ 접근 가능 (WCAG 2.1 AA)

#### FR-P2-005: Multi-Agent Memory Sharing (다중 에이전트 메모리 공유)

**설명**: 여러 OpenClaw 에이전트 인스턴스 간에 메모리를 공유합니다.

**요구사항**:
- 에이전트 범위 메모리 격리
- 교차 에이전트 메모리 공유 (옵트인)
- 동시 업데이트에 대한 충돌 해결
- 에이전트 인증
- 감사 로깅

**승인 기준**:
- ✅ 각 에이전트가 기본적으로 격리된 메모리 보유
- ✅ 올바른 권한으로 공유 메모리 액세스 가능
- ✅ 데이터 손실 없이 동시 업데이트 해결
- ✅ 전체 감사 추적 유지

#### FR-P2-006: Advanced Analytics (고급 분석)

**설명**: 메모리 사용 패턴 및 효과에 대한 인사이트입니다.

**요구사항**:
- 토큰 절감 계산
- 쿼리 빈도 분석
- 관찰 카테고리 분석
- 시간적 사용 패턴
- 압축 효과 메트릭
- CSV/JSON으로 내보내기

**승인 기준**:
- ✅ 분석이 매일 업데이트됨
- ✅ 1년간 히스토리 데이터 유지
- ✅ 내보내기가 10초 이내에 완료
- ✅ 시각화가 명확하고 실행 가능함
---

## 4. User Stories (사용자 스토리)

### 4.1 Core Functionality Stories (핵심 기능 스토리)

#### US-001: 개발자로서, OpenClaw가 프로젝트 제약사항을 기억하기를 원합니다

**스토리**:
> 여러 프로젝트를 진행하는 개발자로서,
> OpenClaw가 내가 언급한 기술적 제약사항을 기억하기를 원합니다,
> 그래서 매 대화마다 반복할 필요가 없습니다.

**승인 기준**:
- OpenClaw에게 "JavaScript 대신 TypeScript를 사용하세요"라고 말하면
- 프로젝트에 대한 새로운 대화를 시작할 때
- OpenClaw가 이 제약사항을 기억하고 적용해야 함
- 그리고 이 선호도를 반복할 필요가 없어야 함

**우선순위**: P0
**예상 공수**: 5 스토리 포인트

#### US-002: 사용자로서, 대화 히스토리를 시맨틱하게 검색하기를 원합니다

**스토리**:
> 수개월의 대화 히스토리를 가진 사용자로서,
> 키워드뿐만 아니라 의미로 주제를 검색하기를 원합니다,
> 그래서 관련된 과거 논의를 쉽게 찾을 수 있습니다.

**승인 기준**:
- 1000개 이상의 관찰이 저장되어 있을 때
- "API 인증 문제"를 검색하면
- 결과에 "로그인 문제" 및 "인증 실패"에 대한 대화가 포함되어야 함
- 그리고 결과가 관련성으로 순위가 매겨져야 함

**우선순위**: P1
**예상 공수**: 8 스토리 포인트

#### US-003: 파워 유저로서, 자동 메모리 아카이빙을 원합니다

**스토리**:
> 대량의 데이터를 생성하는 파워 유저로서,
> 오래된 관찰이 자동으로 아카이브되기를 원합니다,
> 그래서 활성 메모리가 빠르고 관련성 있게 유지됩니다.

**승인 기준**:
- 관찰이 90일보다 오래되었을 때
- 일일 TTL 작업이 실행되면
- 관찰이 웜 메모리로 이동해야 함
- 그리고 핫 메모리가 30MB 미만으로 유지되어야 함

**우선순위**: P1
**예상 공수**: 13 스토리 포인트

### 4.2 Integration Stories (통합 스토리)

#### US-004: 개발자로서, OpenClaw 제로 수정 통합을 원합니다

**스토리**:
> OpenClaw를 사용하는 개발자로서,
> OpenClaw 코드를 수정하지 않고 메모리 기능을 추가하기를 원합니다,
> 그래서 충돌 없이 OpenClaw를 업데이트할 수 있습니다.

**승인 기준**:
- OpenClaw가 설치되어 있을 때
- OC-Memory-Sidecar를 설치하면
- 시스템 프롬프트만을 통해 메모리 기능이 추가되어야 함
- 그리고 OpenClaw 핵심 파일이 수정되지 않아야 함

**우선순위**: P0
**예상 공수**: 3 스토리 포인트

#### US-005: 사용자로서, 메모리가 클라우드에 백업되기를 원합니다

**스토리**:
> 데이터 손실을 우려하는 사용자로서,
> 장기 메모리가 자동으로 백업되기를 원합니다,
> 그래서 중요한 대화 히스토리를 잃지 않습니다.

**승인 기준**:
- Dropbox 통합을 구성했을 때
- 관찰이 콜드 메모리로 이동하면
- Dropbox에 자동으로 동기화되어야 함
- 그리고 확인 알림을 받아야 함

**우선순위**: P2
**예상 공수**: 8 스토리 포인트

### 4.3 Usability Stories (사용성 스토리)

#### US-006: 사용자로서, 메모리가 사용되는 시점을 알고 싶습니다

**스토리**:
> OpenClaw 사용자로서,
> 메모리가 액세스되는 시점에 대한 가시성을 원합니다,
> 그래서 시스템이 어떻게 작동하는지 이해할 수 있습니다.

**승인 기준**:
- OpenClaw가 내 쿼리에 응답할 때
- 컨텍스트를 위해 메모리가 검색되면
- 미묘한 표시기를 보아야 함
- 그리고 표시기가 대화를 방해하지 않아야 함

**우선순위**: P2
**예상 공수**: 5 스토리 포인트

#### US-007: 관리자로서, 메모리 수명주기를 관리하기를 원합니다

**스토리**:
> 시스템 관리자로서,
> 메모리 아카이빙 및 삭제에 대한 제어를 원합니다,
> 그래서 데이터 보존 정책을 준수할 수 있습니다.

**승인 기준**:
- 다양한 계층에 관찰이 있을 때
- 아카이브 관리를 위한 CLI 명령을 실행하면
- 아카이빙 작업을 보고, 승인하거나 거부할 수 있어야 함
- 그리고 사용자 정의 보존 기간을 설정할 수 있어야 함

**우선순위**: P1
**예상 공수**: 8 스토리 포인트

### 4.4 Performance Stories (성능 스토리)

#### US-008: 사용자로서, 빠른 대화 응답을 원합니다

**스토리**:
> 메모리가 활성화된 OpenClaw 사용자로서,
> 대화 응답이 빠르게 유지되기를 원합니다,
> 그래서 메모리가 워크플로우를 느리게 하지 않습니다.

**승인 기준**:
- OpenClaw에 메시지를 보낼 때
- 메모리 검색이 트리거되면
- 메모리 로드 시간이 1초 미만이어야 함
- 그리고 응답 지연이 10% 이상 증가하지 않아야 함

**우선순위**: P0
**예상 공수**: 5 스토리 포인트

#### US-009: 파워 유저로서, 효율적인 토큰 사용을 원합니다

**스토리**:
> API 비용을 지불하는 파워 유저로서,
> 메모리 압축이 토큰 사용량을 극적으로 줄이기를 원합니다,
> 그래서 비용을 관리할 수 있습니다.

**승인 기준**:
- 100,000 토큰의 대화 히스토리가 있을 때
- 메모리 압축이 적용되면
- 활성 메모리가 10,000 토큰 미만이어야 함
- 그리고 90% 이상의 토큰 절감을 확인해야 함

**우선순위**: P0
**예상 공수**: 13 스토리 포인트

#### US-010: 신규 사용자로서, 대화형 설정 마법사를 원합니다

**스토리**:
> OC-Memory-Sidecar를 설치하는 신규 사용자로서,
> 구성을 안내하는 대화형 설정 마법사를 원합니다,
> 그래서 문서를 읽지 않고 빠르게 시작할 수 있습니다.

**승인 기준**:
- 처음으로 설정 마법사를 실행할 때
- 마법사의 6단계를 모두 완료하면
- 내 설정으로 config.yaml이 생성되어야 함
- 그리고 .env 파일에 내 API 키가 안전하게 포함되어야 함
- 그리고 즉시 시스템 사용을 시작할 수 있어야 함
- 그리고 선택적 기능(Obsidian/Dropbox)을 활성화하거나 건너뛸 수 있어야 함

**우선순위**: P0
**예상 공수**: 8 스토리 포인트

### 4.5 Error Handling Stories (에러 처리 스토리)

#### US-011: 에러 복구 시스템

**스토리**:
> OpenClaw 사용자로서,
> LLM 압축 실패 시 자동으로 알림 받기를 원합니다,
> 그래서 메모리 시스템 장애를 빠르게 인지하고 대응할 수 있습니다.

**승인 기준**:
- 관찰 생성 LLM API가 3회 실패할 때
- 시스템이 자동으로 재시도를 수행하면
- Telegram으로 에러 메시지를 받아야 함
- 그리고 에러 원인을 파악할 수 있어야 함 (토큰, 타임아웃 등)
- 그리고 수동 개입 없이 자동 복구를 시도해야 함
- 그리고 낮은 품질의 압축으로 사용자가 갇히지 않아야 함

**우선순위**: P0
**예상 공수**: 8 스토리 포인트

### 4.6 Error Notification and Recovery System (에러 알림 및 복구 시스템)

**기능 개요**:
- Observer/Reflector LLM API 호출 실패 시 자동 재시도 및 OpenClaw 알림
- Zero-Core-Modification 원칙 유지
- HTTP API Hook 방식으로 OpenClaw에 알림 전송

**상세 요구사항**:

#### FR-P0-008: Retry Policy (재시도 정책)
- **우선순위**: P0 (Critical)
- **설명**: LLM API 호출 실패 시 자동 재시도
- **상세**:
  - 재시도 횟수: 3회
  - 대기 시간: 지수 백오프 (2초, 4초, 8초)
  - tenacity 라이브러리 사용
  - 각 재시도마다 로그 기록
- **Acceptance Criteria**:
  - [ ] 3회 재시도 동작 확인
  - [ ] 지수 백오프 정확성
  - [ ] 모든 재시도 로그 기록
  - [ ] 재시도 성공 시 정상 처리

#### FR-P0-009: HTTP API Hook 알림
- **우선순위**: P0 (Critical)
- **설명**: 3회 재시도 실패 후 OpenClaw에 HTTP API로 알림 전송
- **상세**:
  - Endpoint: `POST /hooks/oc-memory-alert`
  - Payload: JSON 형식 (source, event, severity, details, timestamp)
  - OpenClaw가 Telegram으로 사용자에게 전달
  - Fallback 전략 없음 (품질 저하 방지)
- **Acceptance Criteria**:
  - [ ] HTTP POST 요청 성공
  - [ ] Payload 포맷 정확
  - [ ] OpenClaw 수신 확인
  - [ ] Telegram 알림 수신 (사용자 확인)

#### FR-P0-010: OpenClaw API 자동 탐지
- **우선순위**: P0 (Critical)
- **설명**: OpenClaw HTTP API 엔드포인트를 자동으로 탐지하거나 쉽게 설정
- **상세**:
  - 우선순위 1: OpenClaw config.yaml 파싱
  - 우선순위 2: 환경 변수 (OPENCLAW_API_URL)
  - 우선순위 3: 프로세스 포트 스캔 (psutil)
  - 우선순위 4: 기본 포트 테스트 (8080, 8000, 3000)
  - 우선순위 5: TUI에서 수동 입력
- **Acceptance Criteria**:
  - [ ] 자동 탐지 성공률 ≥ 80%
  - [ ] 연결 테스트 성공
  - [ ] TUI 수동 입력 동작
  - [ ] config.yaml에 저장

#### FR-P1-008: 압축 품질 검증 (선택사항)
- **우선순위**: P1 (Important)
- **설명**: 압축 후 품질 자동 검증
- **상세**:
  - 주요 키워드 보존율 계산
  - 70% 미만 시 경고 알림
  - 품질 점수 로그 기록
- **Acceptance Criteria**:
  - [ ] 키워드 추출 정확도 ≥ 85%
  - [ ] 품질 점수 계산 정확
  - [ ] 저품질 시 경고 전송

**Note**: Fallback 전략은 제외 (낮은 품질의 압축으로 사용자가 갇히는 문제 방지)

---

## 5. Functional Requirements (기능 요구사항 상세)

### 5.1 Log Monitoring Module (로그 모니터링 모듈)

**컴포넌트**: `LogWatcher`

**기능**:
- 구성 가능한 간격(기본값: 1초)으로 로그 파일 폴링
- 재시작 간 읽기 위치 유지
- 파일 로테이션 및 절단 처리
- 메시지 쌍(사용자 + 어시스턴트) 추출
- 입력 데이터에 대한 토큰 수 계산
- 임계값 도달 시 관찰 생성 트리거

**인터페이스**:
```python
class LogWatcher:
    def __init__(log_path: str, state_path: str)
    def get_new_lines() -> Tuple[List[str], int]
    def tail(callback: callable, interval: float)
    def _load_state() -> dict
    def _save_state()
```

**의존성**:
- Python `pathlib`
- 토큰 카운팅을 위한 `tiktoken`

### 5.2 Observation Generation Module (관찰 생성 모듈)

**컴포넌트**: `Observer`

**기능**:
- LLM을 사용한 대화 로그 분석
- 구조화된 관찰 생성
- 우선순위 분류 (높음/중간/낮음)
- 유형별 분류 (상태/결정/선호도/작업)
- 시간 정보 추출
- 고유 ID 할당
- 마크다운 형식화

**인터페이스**:
```python
class Observer:
    def __init__(model: str, api_key: str)
    def observe(messages: List[str]) -> List[Observation]
    def _build_prompt(messages: List[str]) -> str
    def _parse_response(response: str) -> List[Observation]
```

**시스템 프롬프트 요구사항**:
- 관찰의 구체성 강조
- 사용자 주장과 질문 구분
- 시간적 앵커링 보존
- 일관된 형식 유지

**의존성**:
- OpenAI 호환 LLM API
- Observation 구조를 위한 `dataclasses`

### 5.3 Memory Storage Module (메모리 저장소 모듈)

**컴포넌트**: `MemoryStore` (ChromaDB)

**기능**:
- 임베딩과 함께 관찰 지속
- 유사도 검색 지원
- 메타데이터로 필터링 (날짜, 우선순위, 카테고리)
- 순위가 매겨진 결과 반환
- TTL을 위한 연령별 관찰 쿼리
- ID별 관찰 삭제

**인터페이스**:
```python
class MemoryStore:
    def __init__(db_path: str)
    def add_observation(observation: Observation)
    def search(query: str, n_results: int, where: dict) -> List[dict]
    def get_older_than(days: int) -> List[str]
    def delete(ids: List[str])
```

**저장 형식**:
- 문서: 관찰 콘텐츠 텍스트
- 임베딩: ChromaDB에 의해 자동 생성
- 메타데이터: 타임스탬프, 우선순위, 카테고리가 포함된 JSON

**의존성**:
- `chromadb` (영구 클라이언트)

### 5.4 Memory File Manager (메모리 파일 매니저)

**컴포넌트**: `MemoryFileWriter`

**기능**:
- `active_memory.md` 파일 생성
- 마크다운으로 관찰 형식화
- 토큰 예산 유지 (구성 가능한 제한)
- 메타데이터 헤더 포함 (마지막 업데이트, 토큰 수)
- 원자적 파일 쓰기 (손상 방지)
- 이전 버전 아카이브 (선택사항)

**파일 형식**:
```markdown
# Active Memory - OpenClaw Agent

> Last Updated: 2026-02-12 17:00 KST
> Total Tokens: ~2,500

## 📋 현재 컨텍스트
**활성 작업**: [관찰에서 가져온 최신 작업]
**상태**: [현재 상태]

## 📝 관찰 로그
날짜: 2026-02-12
* 🔴 09:15 사용자가 TypeScript 선호도를 언급함
* 🟡 10:30 사용자가 API 인증에 대해 질문함

## ⚙️ 사용자 제약사항
- TypeScript를 사용해야 함
- 이 프로젝트에서 React 피하기
```

**인터페이스**:
```python
class MemoryFileWriter:
    def __init__(file_path: str, max_tokens: int)
    def update(observations: List[Observation])
    def _format_markdown(observations: List[Observation]) -> str
    def _count_tokens(content: str) -> int
```

**의존성**:
- 토큰 카운팅을 위한 `tiktoken`
- Python 파일 I/O

### 5.5 Reflection Module (반영 모듈)

**컴포넌트**: `Reflector`

**기능**:
- 관찰 배치 압축
- 상위 수준 요약 생성
- 토큰을 줄이면서 핵심 사실 보존
- 시간적 컨텍스트 유지
- 관찰 유형 구분
- 압축된 형식 출력

**인터페이스**:
```python
class Reflector:
    def __init__(model: str, api_key: str)
    def reflect(observations: List[Observation]) -> str
    def _build_prompt(observations: List[Observation]) -> str
    def _parse_response(response: str) -> str
```

**압축 전략**:
- 관련 관찰 결합
- 모든 높은 우선순위 항목 보존
- 카테고리별로 낮은 우선순위 항목 집계
- 시간순 유지

**의존성**:
- OpenAI 호환 LLM API

### 5.6 TTL Management Module (TTL 관리 모듈)

**컴포넌트**: `TTLManager`

**기능**:
- 일일 TTL 체크 스케줄링
- 핫 TTL(90일)을 초과하는 관찰 식별
- 웜 메모리로 아카이브 (마크다운 파일)
- 웜 → 콜드 전환에 대한 사용자 승인 요청
- 아카이빙 작업 실행
- 모든 전환 로깅

**인터페이스**:
```python
class TTLManager:
    def __init__(memory_store: MemoryStore, archive_path: str)
    def check_and_archive() -> dict
    def _archive_to_warm(observation_id: str)
    def _archive_to_cold(date_range: tuple) -> bool
```

**아카이빙 프로세스**:
1. TTL보다 오래된 관찰 쿼리
2. 전체 관찰 데이터 검색
3. 마크다운 아카이브 파일에 쓰기
4. 핫 메모리(ChromaDB)에서 삭제
5. 아카이브 인덱스 업데이트

**의존성**:
- `MemoryStore`
- 파일시스템 액세스
- 선택사항: Obsidian CLI

### 5.7 Obsidian Integration Module (Obsidian 통합 모듈)

**컴포넌트**: `ObsidianIntegration`

**기능**:
- Obsidian 볼트에 관찰 아카이브
- 적절하게 형식화된 마크다운 노트 생성
- 프론트매터 메타데이터 적용 (태그, 날짜, 유형)
- Obsidian CLI 명령 실행
- Dropbox 동기화 지원
- REST API를 통한 역 쿼리 활성화

**인터페이스**:
```python
class ObsidianIntegration:
    def __init__(vault_name: str, dropbox_token: str, api_key: str)
    def archive_memory(date: str, content: str, tags: list)
    def read_from_dropbox(date: str) -> Optional[str]
    def read_from_obsidian_rest(note_path: str) -> Optional[str]
    def search_cloud_memories(query: str) -> List[str]
```

**의존성**:
- Obsidian CLI (Yakitrak)
- Dropbox API SDK
- Obsidian REST API를 위한 `requests`

### 5.8 CLI Interface (CLI 인터페이스)

**컴포넌트**: `oc-memory` CLI

**명령어**:
```bash
oc-memory start              # 데몬 시작
oc-memory stop               # 데몬 중지
oc-memory status             # 시스템 상태 표시
oc-memory search <query>     # 메모리 검색
oc-memory archive [options]  # 수동 아카이브
oc-memory stats              # 통계 표시
oc-memory config <key> <val> # 설정 구성
```

**상태 출력 예시**:
```
OC-Memory 상태:
  데몬: ✅ 실행 중 (PID 12345)
  핫 메모리: 2,345개 관찰 (25.3 MB)
  웜 메모리: 8,123개 관찰 (78.1 MB)
  콜드 메모리: 23,456개 관찰 (Obsidian)
  마지막 업데이트: 2분 전
  토큰 절감: 92.3%
```

**의존성**:
- CLI를 위한 `click` 또는 `argparse`
- 시스템 프로세스 관리
---

## 6. Non-Functional Requirements (비기능 요구사항)

### 6.1 Performance Requirements (성능 요구사항)

| 요구사항 | 목표 | 측정 방법 |
|----------|------|-----------|
| **로그 감지 지연** | ≤1초 | 로그 쓰기부터 감지까지의 시간 |
| **관찰 생성** | ≤5초 | 감지부터 저장까지의 시간 |
| **메모리 파일 업데이트** | ≤2초 | active_memory.md 쓰기 시간 |
| **검색 쿼리 응답** | ≤500ms | 쿼리부터 결과까지의 시간 |
| **메모리 로드 시간** | ≤1초 | OpenClaw가 메모리를 읽는 시간 |
| **CPU 사용량 (유휴)** | ≤5% | 폴링 중 시스템 모니터 |
| **CPU 사용량 (활성)** | ≤30% | 관찰 중 시스템 모니터 |
| **메모리 사용량** | ≤512MB RAM | 프로세스 메모리 풋프린트 |

### 6.2 Scalability Requirements (확장성 요구사항)

| 측면 | 사양 | 근거 |
|------|------|------|
| **최대 관찰 수 (핫)** | 100,000개 | ChromaDB 성능 한계 |
| **일일 최대 대화 수** | 500개 | 파워 유저 시나리오 |
| **일일 최대 토큰 입력** | 500,000개 | 일반적인 고사용량 |
| **ChromaDB 크기 제한** | 500MB | 디스크 공간 고려사항 |
| **아카이브 파일 수** | 1,000개 파일 | 파일시스템 성능 |
| **동시 사용자** | 1명 (단일 에이전트) | MVP 범위 |

### 6.3 Reliability Requirements (신뢰성 요구사항)

| 요구사항 | 사양 | 구현 |
|----------|------|------|
| **가동 시간** | 99.9% | 데몬 자동 재시작 |
| **데이터 내구성** | 99.99% | 원자적 쓰기, 백업 |
| **크래시 복구** | 자동 | 상태 파일 지속성 |
| **데이터 손실 방지** | 무관용 | 중복 저장소 |
| **백업 빈도** | 일일 | 자동화된 스크립트 |

### 6.4 Security Requirements (보안 요구사항)

| 요구사항 | 사양 | 구현 |
|----------|------|------|
| **저장 데이터 암호화** | 선택사항 | 파일시스템 암호화 |
| **API 키 저장** | 환경 변수만 사용 | 코드나 설정에 절대 포함 안 함 |
| **파일 권한** | 600 (소유자만 읽기/쓰기) | 파일 생성 시 자동 |
| **네트워크 전송** | 로컬만 (P0), P2의 경우 HTTPS | P0에서 원격 API 없음 |
| **민감 데이터 마스킹** | 자동 감지 | 키, 토큰에 대한 정규식 패턴 |
| **감사 로깅** | 모든 아카이빙 작업 | 추가 전용 로그 파일 |

**민감 데이터 패턴** (자동 마스킹):
- API 키 (정규식: `[A-Za-z0-9_-]{32,}`)
- 토큰 (정규식: `token[=:]\s*[A-Za-z0-9_-]+`)
- 비밀번호 (정규식: `password[=:]\s*\S+`)
- 이메일 주소 (구성 가능)

### 6.5 Maintainability Requirements (유지보수성 요구사항)

| 요구사항 | 사양 | 방법 |
|----------|------|------|
| **코드 문서화** | 100% 공개 API | Docstring (Google 스타일) |
| **테스트 커버리지** | ≥80% | 커버리지 리포트가 있는 pytest |
| **구성** | YAML 기반 | 단일 설정 파일 |
| **로깅** | 구조화된 로그 | JSON 형식, 다중 레벨 |
| **모듈성** | 플러그형 컴포넌트 | 의존성 주입 |
| **버전 관리** | 시맨틱 버전 관리 | CalVer (YYYY.MM.PATCH) |

### 6.6 Usability Requirements (사용성 요구사항)

| 요구사항 | 사양 | 성공 지표 |
|----------|------|-----------|
| **설치 시간** | ≤5분 | 사용자 테스트 시간 측정 |
| **구성 복잡도** | 시작하기 위한 ≤10개 설정 | 설정 파일 크기 |
| **문서 완성도** | 100% 기능 문서화 | 문서 검토 |
| **오류 메시지 명확성** | 평이한 언어, 실행 가능 | 사용자 피드백 |
| **CLI 도움말 접근성** | 모든 명령에 --help | 자동 체크 |

### 6.7 Compatibility Requirements (호환성 요구사항)

| 컴포넌트 | 요구사항 | 버전 |
|----------|----------|------|
| **Python** | Python 3.8+ | 3.8, 3.9, 3.10, 3.11, 3.12 |
| **OpenClaw** | v2026.2.x | 2026.2.1 - 2026.2.6+ |
| **운영체제** | macOS, Linux, Windows | macOS 12+, Ubuntu 20.04+, Windows 10+ |
| **LLM API** | OpenAI 호환 | OpenAI, Anthropic, Google, OpenRouter |
| **Obsidian** | 선택사항, 모든 버전 | 1.0.0+ |

### 6.8 Compliance Requirements (규정 준수 요구사항)

| 요구사항 | 사양 | 증거 |
|----------|------|------|
| **데이터 프라이버시** | GDPR 준수 (로컬 저장소) | 개인정보 보호 정책 |
| **오픈소스 라이선스** | MIT 또는 Apache 2.0 | LICENSE 파일 |
| **데이터 이식성** | 표준 형식으로 내보내기 | 내보내기 기능 |
| **삭제 권리** | 사용자가 모든 데이터 삭제 가능 | 삭제 명령 |

---

## 7. Data Model (데이터 모델)

### 7.1 Core Data Structures (핵심 데이터 구조)

#### 7.1.1 Observation (관찰)

```python
@dataclass
class Observation:
    """
    대화에서 추출된 단일 관찰을 나타냅니다.
    """
    id: str                    # UUID v4
    timestamp: datetime        # ISO 8601 형식
    priority: Literal['high', 'medium', 'low']
    category: Literal['state', 'decision', 'preference', 'task']
    content: str               # 실제 관찰 텍스트
    metadata: dict             # 추가 컨텍스트

    # 선택적 필드
    source_file: Optional[str] = None      # 원본 로그 파일
    thread_id: Optional[str] = None        # 대화 스레드
    user_id: Optional[str] = None          # 사용자 식별자
    tags: Optional[List[str]] = None       # 사용자 정의 태그
    embedding: Optional[List[float]] = None # 벡터 임베딩
```

**예시**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2026-02-12T09:15:00Z",
  "priority": "high",
  "category": "preference",
  "content": "사용자가 백엔드 개발에 JavaScript보다 TypeScript를 선호한다고 언급함",
  "metadata": {
    "source_file": "~/.openclaw/logs/chat.log",
    "thread_id": "thread_abc123",
    "confidence": 0.95
  },
  "tags": ["typescript", "preference", "backend"]
}
```

#### 7.1.2 Reflection (반영)

```python
@dataclass
class Reflection:
    """
    여러 관찰의 압축된 요약입니다.
    """
    id: str                    # UUID v4
    created_at: datetime       # 생성 시간
    period_start: datetime     # 관찰 기간 시작
    period_end: datetime       # 관찰 기간 종료
    content: str               # 압축된 요약
    source_observations: List[str]  # 소스 관찰의 ID
    compression_ratio: float   # 토큰 압축 비율
    metadata: dict
```

**예시**:
```json
{
  "id": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "created_at": "2026-02-12T18:00:00Z",
  "period_start": "2026-02-12T08:00:00Z",
  "period_end": "2026-02-12T17:00:00Z",
  "content": "사용자가 TypeScript와 PostgreSQL을 사용하여 마이크로서비스 프로젝트를 진행함...",
  "source_observations": ["550e8400...", "661f9511...", "..."],
  "compression_ratio": 12.5,
  "metadata": {
    "observation_count": 45,
    "token_before": 5000,
    "token_after": 400
  }
}
```

#### 7.1.3 Memory State (메모리 상태)

```python
@dataclass
class MemoryState:
    """
    재시작 간 지속성을 위한 시스템 상태입니다.
    """
    last_log_position: int          # 파일 시크 위치
    last_observation_time: datetime # 가장 최근 관찰
    total_observations: int         # 핫 메모리의 개수
    total_archived: int             # 웜/콜드의 개수
    token_count_active: int         # active_memory.md의 토큰
    last_ttl_run: datetime          # 마지막 TTL 체크
    last_reflection: datetime       # 마지막 반영 작업
    version: str                    # 상태 스키마 버전
```

**저장**: `~/.openclaw/.memory_state.json`의 JSON 파일

### 7.2 Database Schema (ChromaDB) (데이터베이스 스키마)

ChromaDB는 스키마가 없지만, 구조를 정의합니다:

**컬렉션**: `observations`

**문서**: 관찰 콘텐츠 문자열

**임베딩**: ChromaDB에 의해 자동 생성 (OpenAI 임베딩의 경우 1536차원 벡터)

**메타데이터**:
```json
{
  "id": "UUID",
  "timestamp": "ISO 8601 문자열",
  "priority": "high|medium|low",
  "category": "state|decision|preference|task",
  "source_file": "string",
  "thread_id": "string",
  "user_id": "string",
  "tags": ["tag1", "tag2"]
}
```

**인덱스**:
- 임베딩에 대한 HNSW 인덱스 (코사인 유사도)
- 타임스탬프에 대한 B-트리 인덱스 (TTL 쿼리용)

### 7.3 File System Structure (파일 시스템 구조)

```
~/.openclaw/
├── oc-memory/
│   ├── config.yaml                # 구성 파일
│   ├── memory_observer.py         # 메인 데몬 스크립트
│   ├── lib/                       # Python 모듈
│   │   ├── __init__.py
│   │   ├── watcher.py
│   │   ├── observer.py
│   │   ├── reflector.py
│   │   ├── merger.py
│   │   ├── memory_store.py
│   │   ├── ttl_manager.py
│   │   └── obsidian_integration.py
│   ├── prompts/
│   │   ├── observer_system.md     # 관찰자 시스템 프롬프트
│   │   └── reflector_system.md    # 반영자 시스템 프롬프트
│   └── logs/
│       ├── observer.log           # 관찰자 에이전트 로그
│       ├── reflector.log          # 반영자 에이전트 로그
│       └── error.log              # 오류 로그
├── memory_db/                     # ChromaDB 저장소 (핫 메모리)
│   ├── chroma.sqlite3
│   └── [임베딩 파일들]
├── memory_archive/                # 웜 메모리 (마크다운)
│   ├── 2026-02-12.md
│   ├── 2026-02-11.md
│   └── ...
├── active_memory.md               # 활성 메모리 파일 (OpenClaw가 읽음)
└── .memory_state.json             # 영구 상태
```

### 7.4 Active Memory File Format (활성 메모리 파일 형식)

**파일**: `~/.openclaw/active_memory.md`

**구조**:
```markdown
# Active Memory - OpenClaw Agent

> 마지막 업데이트: 2026-02-12 17:00:00 KST
> 총 토큰: ~2,847
> 관찰 개수: 47
> 메모리 기간: 2026-01-13 ~ 2026-02-12

---

## 📋 현재 컨텍스트

**활성 작업**: 전자상거래 플랫폼을 위한 마이크로서비스 아키텍처 구축
**상태**: 진행 중 - 인증 서비스 구현 중
**마지막 업데이트**: 2026-02-12 16:45

---

## 📝 최근 관찰

### 날짜: 2026-02-12
* 🔴 09:15 사용자가 백엔드에 JavaScript보다 TypeScript를 선호한다고 언급
* 🔴 10:30 사용자가 메인 데이터베이스로 PostgreSQL, 캐싱으로 Redis 사용 결정
* 🟡 11:20 사용자가 JWT vs 세션 기반 인증에 대해 질문
* 🔴 11:25 결정: 리프레시 토큰과 함께 JWT 사용
* 🟡 14:30 사용자가 인증 미들웨어 코드 리뷰 요청
* 🟢 15:00 코드 리뷰 완료, 제안된 개선 사항 적용

### 날짜: 2026-02-11
* 🔴 08:00 사용자가 마이크로서비스 프로젝트 시작
* 🟡 09:30 사용자가 서비스 통신 패턴에 대해 질문
* 🔴 10:00 결정: 동기식으로 REST 사용, 비동기식으로 RabbitMQ 사용

---

## ⚙️ 사용자 선호도 및 제약사항

### 개발 선호도
- **언어**: TypeScript (JavaScript보다 강력히 선호)
- **프레임워크**: 백엔드 서비스용 NestJS
- **데이터베이스**: PostgreSQL (주요), Redis (캐싱)
- **코드 스타일**: 2칸 들여쓰기, 세미콜론 필수
- **테스트**: 단위 테스트용 Jest, 통합 테스트용 Supertest

### 프로젝트 제약사항
- 수평 확장 지원 필수
- 벤더 종속 없음 (AWS 전용 서비스 피하기)
- 목표: <200ms API 응답 시간
- Kubernetes에 배포

### 커뮤니케이션 스타일
- 간결한 설명 선호
- 코드 예제 좋아함
- 장황한 문서 싫어함

---

## 🎯 활성 목표

1. 2026-02-14까지 인증 서비스 완성
2. 사용자 등록 및 로그인 구현
3. JWT 토큰 생성 및 검증 설정
4. 포괄적인 테스트 작성 (>80% 커버리지)

---

## 📚 주요 사실

- 프로젝트 시작일: 2026-02-11
- 목표 출시일: 2026-03-15
- 팀 규모: 단독 개발자
- 이전 경험: Node.js 5년, TypeScript 2년

---

## 🔗 관련 대화

- [2026-02-10] 기술 스택 선택에 대한 토론
- [2026-02-09] 요구사항 수집 세션
- [2026-01-15] 유사한 프로젝트 완료 (모놀리식 전자상거래 앱)

---

_이 메모리 파일은 OC-Memory-Sidecar에 의해 자동으로 유지됩니다._
_수동으로 편집하지 마십시오._
```

**토큰 관리**:
- 목표: 2,000-4,000 토큰
- 최대: 5,000 토큰 (하드 리미트)
- 초과 시: 반영자를 트리거하여 압축
---

## 8. User Interface (CLI) (사용자 인터페이스 - CLI)

### 8.1 CLI Commands (CLI 명령어)

#### 8.1.0 Interactive Setup Wizard (대화형 설정 마법사)

```bash
# 대화형 설정 마법사 실행
python setup.py

옵션:
  --reconfigure        기존 구성을 업데이트하기 위해 설정 재실행
  --verbose            상세 진행 정보 표시

예시:
  $ python setup.py

  ╔══════════════════════════════════════════════════════════╗
  ║     OC-Memory-Sidecar 대화형 설정 마법사                ║
  ╚══════════════════════════════════════════════════════════╝

  환영합니다! 이 마법사가 OpenClaw용 OC-Memory-Sidecar
  설정을 안내합니다.

  설정은 약 3-5분이 소요됩니다.

  계속하려면 Enter를 누르세요...
```

**6단계 설정 프로세스**:

**1단계: 기본 구성**
```
─────────────────────────────────────────────────────────────
1단계/6단계: 기본 구성
─────────────────────────────────────────────────────────────

? 설치 디렉토리: [~/.openclaw/oc-memory]
? 활성 메모리 토큰 제한: [4000]
? 로그 폴링 간격 (초): [1]

✅ 기본 구성 완료
```

**2단계: 메모리 계층 구성**
```
─────────────────────────────────────────────────────────────
2단계/6단계: 메모리 계층 구성
─────────────────────────────────────────────────────────────

자동 아카이빙 기간 구성:

? 핫 메모리 TTL (일): [90]
  이보다 최신 관찰은 빠른 저장소에 유지됩니다

? 웜 메모리 TTL (일): [365]
  핫과 콜드 기간 사이의 관찰

? 자동 핫→웜 아카이빙 활성화?: [예]
? 웜→콜드 아카이빙에 승인 필요?: [예]

✅ 메모리 계층 구성 완료
```

**3단계: LLM 공급자 구성**
```
─────────────────────────────────────────────────────────────
3단계/6단계: LLM 공급자 구성
─────────────────────────────────────────────────────────────

관찰자 및 반영자 에이전트용 LLM 공급자 선택:

? 공급자 선택:
  ❯ Google Gemini (권장 - 빠르고 저렴)
    OpenAI GPT
    Anthropic Claude
    OpenRouter (다중 공급자)

? 관찰자 모델: [google/gemini-2.5-flash]
? 반영자 모델: [google/gemini-2.5-flash]

? Google API 키: ••••••••••••••••••••••••••
  (API 키는 .env 파일에 안전하게 저장됩니다)

⏳ API 키 검증 중... ✅ 유효함

✅ LLM 공급자 구성 완료
```

**4단계: 선택적 기능 - Obsidian 통합 (P2)**
```
─────────────────────────────────────────────────────────────
4단계/6단계: 선택적 기능 - Obsidian 통합
─────────────────────────────────────────────────────────────

Obsidian은 장기 콜드 메모리 저장소 및 시각화를 제공합니다.

? Obsidian 통합 활성화?: [예/아니오]

  > 예 선택됨

? Obsidian 볼트 이름: [Main]
? Obsidian CLI 설치됨?: [예/아니오]

  > 아니오 선택됨

  ℹ️  Obsidian CLI를 찾을 수 없습니다. 설치:
     brew install yakitrak/yakitrak/obsidian-cli

  나중에 실행하여 활성화할 수 있습니다: oc-memory config setup

⚠️  Obsidian 통합이 비활성화됩니다
```

**5단계: 선택적 기능 - Dropbox 클라우드 동기화 (P2)**
```
─────────────────────────────────────────────────────────────
5단계/6단계: 선택적 기능 - Dropbox 클라우드 동기화
─────────────────────────────────────────────────────────────

Dropbox는 콜드 메모리 아카이브에 대한 클라우드 백업을 제공합니다.

? Dropbox 동기화 활성화?: [예/아니오]

  > 아니오 선택됨

  ℹ️  나중에 실행하여 Dropbox를 활성화할 수 있습니다: oc-memory config setup

✅ 선택적 기능 구성 완료
```

**6단계: 검토 및 확인**
```
─────────────────────────────────────────────────────────────
6단계/6단계: 구성 검토 및 확인
─────────────────────────────────────────────────────────────

📋 구성 요약:

설치 경로: ~/.openclaw/oc-memory
토큰 제한: 4000 토큰
폴링 간격: 1초

메모리 계층:
  핫 메모리: 0-90일 (자동 아카이브)
  웜 메모리: 90-365일 (수동 승인)
  콜드 메모리: 365일 이상

LLM 공급자:
  공급자: Google Gemini
  관찰자: google/gemini-2.5-flash
  반영자: google/gemini-2.5-flash

선택적 기능:
  Obsidian: ❌ 비활성화
  Dropbox: ❌ 비활성화

? 설치를 진행하시겠습니까?: [예/아니오]

  > 예

⏳ 디렉토리 생성 중...
⏳ config.yaml 생성 중...
⏳ .env 파일 생성 중...
⏳ 파일 권한 설정 중...
⏳ 구성 검증 중...

✅ 설정 완료!

다음 단계:
  1. OpenClaw 시스템 프롬프트에 메모리 지침 추가
     참조: ~/.openclaw/oc-memory/docs/integration.md

  2. 메모리 데몬 시작:
     oc-memory start --daemon

  3. 상태 확인:
     oc-memory status

도움말: oc-memory --help
```

**기능**:
- 색상 코드 출력 (성공: 녹색, 경고: 노란색, 오류: 빨간색)
- 긴 작업에 대한 진행 표시기
- 명확한 오류 메시지가 있는 입력 검증
- 대괄호로 표시된 기본값
- 선택적 기능 건너뛰기 가능
- 터미널에서 API 키 마스킹
- 보안 권한(600)으로 .env 파일 자동 생성
- 사용자 설정으로 config.yaml 생성
- 확인 전 최종 요약

#### 8.1.1 Daemon Management (데몬 관리)

```bash
# 메모리 데몬 시작
oc-memory start [--config PATH] [--daemon]

옵션:
  --config PATH    설정 파일 경로 (기본값: ~/.openclaw/oc-memory/config.yaml)
  --daemon         백그라운드에서 실행 (기본값: 포그라운드)
  --verbose        상세 로깅 활성화

예시:
  $ oc-memory start --daemon
  ✅ OC-Memory 데몬 시작됨 (PID 12345)
```

```bash
# 데몬 중지
oc-memory stop [--force]

옵션:
  --force          정상 종료 실패 시 강제 종료

예시:
  $ oc-memory stop
  ✅ OC-Memory 데몬 중지됨
```

```bash
# 데몬 상태 확인
oc-memory status [--json]

옵션:
  --json           JSON 형식으로 출력

예시:
  $ oc-memory status

  OC-Memory 상태:
  ─────────────────────────────────────
  데몬: ✅ 실행 중 (PID 12345)
  가동 시간: 3일 12시간

  핫 메모리:
    관찰: 2,345개
    크기: 25.3 MB
    가장 오래됨: 12일 전

  웜 메모리:
    관찰: 8,123개
    크기: 78.1 MB
    파일: 78개

  콜드 메모리:
    관찰: 23,456개
    위치: Obsidian Vault
    동기화됨: ✅ Dropbox

  성능:
    마지막 업데이트: 2분 전
    평균 검색 시간: 320ms
    토큰 절감: 92.3%
    CPU 사용량: 4.2%
```

#### 8.1.2 Memory Operations (메모리 작업)

```bash
# 메모리 검색
oc-memory search <query> [--limit N] [--priority LEVEL] [--category CAT]

옵션:
  --limit N            최대 결과 수 (기본값: 10)
  --priority LEVEL     우선순위로 필터링 (high/medium/low)
  --category CAT       카테고리로 필터링 (state/decision/preference/task)
  --since DATE         날짜 이후의 결과만 (YYYY-MM-DD)
  --until DATE         날짜 이전의 결과만 (YYYY-MM-DD)
  --json               JSON 형식으로 출력

예시:
  $ oc-memory search "API 인증"

  검색 결과 (5개 발견):
  ─────────────────────────────────────
  1. 🔴 2026-02-12 11:25
     카테고리: decision
     결정: 인증을 위해 리프레시 토큰과 함께 JWT 사용
     [관련성: 0.94]

  2. 🟡 2026-02-12 11:20
     카테고리: state
     사용자가 JWT vs 세션 기반 인증에 대해 질문함
     [관련성: 0.89]

  3. 🔴 2026-02-10 14:30
     카테고리: preference
     사용자가 마이크로서비스에 스테이트리스 인증 선호
     [관련성: 0.76]
```

```bash
# 수동 아카이브 작업
oc-memory archive [--older-than DAYS] [--to TIER] [--dry-run]

옵션:
  --older-than DAYS    N일보다 오래된 관찰 아카이브
  --to TIER            대상 계층 (warm/cold)
  --dry-run            실행 없이 아카이브될 내용 표시
  --approve-all        웜→콜드 전환 자동 승인

예시:
  $ oc-memory archive --older-than 90 --to warm --dry-run

  아카이브 미리보기 (시험 실행):
  ─────────────────────────────────────
  핫 → 웜: 145개 관찰 (5.2 MB)
  날짜 범위: 2025-11-14 ~ 2025-11-20

  토큰 절감: 15,230 토큰 확보

  실행하려면 --dry-run 없이 실행하세요.
```

#### 8.1.3 Statistics & Analytics (통계 및 분석)

```bash
# 메모리 통계 표시
oc-memory stats [--detailed] [--export PATH]

옵션:
  --detailed           상세 분석 표시
  --export PATH        CSV/JSON 파일로 내보내기

예시:
  $ oc-memory stats --detailed

  OC-Memory 통계:
  ═════════════════════════════════════

  총 관찰: 33,924개
  총 저장공간: 128.6 MB
  메모리 기간: 347일

  계층 분석:
  ┌──────────┬──────────────┬────────────┬─────────────┐
  │ 계층     │ 관찰         │ 크기       │ 연령 범위   │
  ├──────────┼──────────────┼────────────┼─────────────┤
  │ Hot      │ 2,345        │ 25.3 MB    │ 0-89일      │
  │ Warm     │ 8,123        │ 78.1 MB    │ 90-364일    │
  │ Cold     │ 23,456       │ 25.2 MB    │ 365일 이상  │
  └──────────┴──────────────┴────────────┴─────────────┘

  우선순위 분포:
  🔴 높음: 8,234 (24.3%)
  🟡 중간: 15,678 (46.2%)
  🟢 낮음: 10,012 (29.5%)

  카테고리 분포:
  상태: 12,345 (36.4%)
  결정: 8,234 (24.3%)
  선호도: 7,123 (21.0%)
  작업: 6,222 (18.3%)

  토큰 효율성:
  원본 토큰: 1,234,567
  압축됨: 98,765
  절감: 92.0%

  성능:
  평균 관찰 시간: 4.2초
  평균 검색 시간: 320ms
  압축 비율: 12.5:1
```

#### 8.1.4 Configuration Management (구성 관리)

```bash
# 구성 보기 또는 설정
oc-memory config [get|set] <key> [value]

예시:
  $ oc-memory config get memory.hot.ttl_days
  90

  $ oc-memory config set memory.hot.ttl_days 60
  ✅ 구성 업데이트됨: memory.hot.ttl_days = 60
  ⚠️  변경 사항을 적용하려면 데몬을 재시작하세요.

  $ oc-memory config get observer.model
  google/gemini-2.5-flash
```

```bash
# 구성 검증
oc-memory config validate [--config PATH]

예시:
  $ oc-memory config validate

  구성 검증:
  ─────────────────────────────────────
  ✅ config.yaml 구문 유효함
  ✅ 모든 필수 키 존재
  ✅ API 키 구성됨
  ✅ 파일 경로 액세스 가능
  ⚠️  경고: Obsidian CLI를 찾을 수 없음 (기능 비활성화됨)

  결과: 유효함 (경고 1개)
```

### 8.2 Output Formats (출력 형식)

#### 8.2.1 Human-Readable (Default) (사람이 읽을 수 있는 형식 - 기본값)

터미널 보기를 위해 설계됨:
- 색상 코딩 (우선순위: 빨강/노랑/녹색)
- 유니코드 기호 (✅ ❌ ⚠️ 🔴 🟡 🟢)
- 표 (박스 그리기 문자 사용)
- 진행 표시기

#### 8.2.2 JSON Format (JSON 형식)

스크립팅을 위한 기계 판독 가능:
```bash
$ oc-memory status --json
{
  "daemon": {
    "running": true,
    "pid": 12345,
    "uptime_seconds": 302400
  },
  "hot_memory": {
    "observations": 2345,
    "size_bytes": 26522931,
    "oldest_age_days": 12
  },
  "warm_memory": {
    "observations": 8123,
    "size_bytes": 81919180,
    "file_count": 78
  },
  "cold_memory": {
    "observations": 23456,
    "location": "obsidian",
    "synced": true
  },
  "performance": {
    "last_update_seconds_ago": 120,
    "avg_search_ms": 320,
    "token_savings_percent": 92.3,
    "cpu_percent": 4.2
  }
}
```

### 8.3 Error Handling (오류 처리)

모든 CLI 명령은 일관된 오류 패턴을 따릅니다:

**종료 코드**:
- `0`: 성공
- `1`: 일반 오류
- `2`: 구성 오류
- `3`: 데몬이 실행되지 않음
- `4`: 권한 거부됨
- `5`: API 오류

**오류 메시지 형식**:
```
❌ 오류: <오류 유형>

<무엇이 잘못되었는지에 대한 상세 설명>

제안: <실행 가능한 조언>

추가 도움말: oc-memory help <command>
```

**예시**:
```bash
$ oc-memory search "test"
❌ 오류: 데몬이 실행되지 않음

OC-Memory 데몬이 현재 실행되지 않습니다.
메모리 검색을 위해서는 데몬이 활성화되어 있어야 합니다.

제안: 'oc-memory start --daemon'으로 데몬을 시작하세요

추가 도움말: oc-memory help search
```
---

## 9. Integration Requirements (통합 요구사항)

### 9.1 OpenClaw Integration (OpenClaw 통합)

#### 9.1.1 통합 방법 개요

OpenClaw는 다양한 확장 메커니즘을 제공하며, OC-Memory는 다음 방법들을 활용합니다:

**방법 A (권장): Memory Files Auto-Indexing**
- 가장 간단하고 안정적
- Zero-Configuration
- OpenClaw 내장 기능 최대 활용

**방법 B: openclaw.json 설정**
- 정적 System Prompt 주입
- Context Files 로드
- Memory 경로 설정

**방법 C: Plugin Hook**
- 동적 Prompt 생성
- 이벤트 기반 처리
- 가장 강력하고 유연함

#### 9.1.2 방법 A: Memory Files Auto-Indexing (권장)

**설정 파일**: `~/.openclaw/openclaw.json`

```json
{
  "agents": {
    "main": {
      "memory": {
        "enabled": true,
        "provider": "openai",
        "model": "text-embedding-3-small",
        "chunkTokens": 512,
        "chunkOverlap": 128,
        "extraPaths": []
      }
    }
  }
}
```

**OC-Memory 작업**:
1. 사용자 노트 디렉토리 감시 (`~/Documents/notes`, `~/Projects`)
2. 중요 파일을 `~/.openclaw/workspace/memory/` 에 복사
3. OpenClaw가 자동으로 파일 감시 (chokidar, 5초 debounce)
4. 자동 인덱싱: SQLite + Vector (sqlite-vec) + FTS5

**에이전트 사용**:
- `memory_search` tool 자동 제공
- 벡터 + 키워드 하이브리드 검색
- 파일 내용 직접 읽기 가능

**장점**:
- ✅ 설정 불필요
- ✅ 자동 인덱싱
- ✅ 검색 성능 우수
- ✅ Zero-Core-Modification

#### 9.1.3 방법 B: openclaw.json 설정

**설정 파일**: `~/.openclaw/openclaw.json`

```json
{
  "agents": {
    "main": {
      "systemPrompt": "You have access to OC-Memory system.\n\nRecent user context:\n- Prefers Python for scripting\n- Working on AI projects\n- Uses VS Code\n\nAlways check memory files before responding.",

      "contextFiles": [
        "~/oc-memory/active_memory.md",
        "~/oc-memory/user_preferences.md"
      ],

      "workspace": {
        "dir": "~/.openclaw/workspace",
        "bootstrapFiles": [
          "~/oc-memory/initialization.md"
        ]
      }
    }
  }
}
```

**특징**:
- 정적 System Prompt
- 파일 내용 자동 로드
- 세션 시작 시 Bootstrap 파일 실행

#### 9.1.4 방법 C: Plugin Hook (고급)

**Plugin 위치**: `~/.openclaw/plugins/oc-memory/index.js`

```javascript
module.exports = {
  name: "oc-memory-integration",
  version: "1.0.0",

  hooks: [
    {
      hookName: "before_agent_start",
      handler: async (event, ctx) => {
        // 동적으로 최근 메모리 로드
        const recentMemories = await loadRecentMemories(5);

        const memoryContext = recentMemories
          .map(m => `- ${m.category}: ${m.summary}`)
          .join('\n');

        return {
          prependContext: `# Recent Memory Context\n\n${memoryContext}\n\n---\n`
        };
      }
    },

    {
      hookName: "after_tool_call",
      handler: async (result, ctx) => {
        // 파일 작성 시 메모리 인덱싱 트리거
        if (result.tool === 'write_file' && result.args.path.includes('.md')) {
          await indexMemoryFile(result.args.path);
        }
      }
    }
  ]
};
```

**장점**:
- ✅ 동적 Prompt 생성
- ✅ 조건부 로직 가능
- ✅ Tool 호출 가로채기
- ✅ 외부 API 호출 가능

#### 9.1.5 Webhook Integration

**설정 파일**: `~/.openclaw/openclaw.json`

```json
{
  "hooks": {
    "enabled": true,
    "token": "your-secret-webhook-token",
    "path": "/hooks",
    "maxBodyBytes": 262144,
    "allowedAgentIds": ["*"]
  }
}
```

**OC-Memory Webhook 전송**:

```bash
# 파일 변경 감지 시
curl -X POST http://localhost:18789/hooks/agent \
  -H "Authorization: Bearer <webhook-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "New memory file detected: projects/AI/research.md",
    "name": "OC-Memory-Watcher",
    "agentId": "main",
    "wakeMode": "now",
    "sessionKey": "external:memory-sync:2026-02-12"
  }'
```

**응답**:
- OpenClaw가 즉시 에이전트 wake
- 메모리 파일 자동 인덱싱
- Telegram 등으로 알림 전송 (선택)

#### 9.1.6 Session Transcript Access

**실제 로그 위치**: `~/.openclaw/agents/<agentId>/sessions/<sessionId>.jsonl`

**로그 형식** (JSONL):
```jsonl
{"type":"user","role":"user","content":"JWT 인증을 어떻게 구현하나요?","timestamp":"2026-02-12T09:15:23.000Z"}
{"type":"assistant","role":"assistant","content":"Let me read the file...","timestamp":"2026-02-12T09:15:24.000Z"}
{"type":"tool_use","tool":"read_file","args":{"path":"/path/to/auth.py"},"timestamp":"2026-02-12T09:15:25.000Z"}
{"type":"tool_result","tool":"read_file","content":"def authenticate...","timestamp":"2026-02-12T09:15:26.000Z"}
[2026-02-12 09:15:31] ASSISTANT: JWT 인증을 구현하려면...
```

**호환성 요구사항**:
- OC-Memory 데몬이 로그 파일을 읽을 수 있어야 함
- 파일 권한: 644 (사용자 읽기/쓰기, 그룹/기타 읽기)
- 로그 로테이션 지원 (OC-Memory가 로테이션 감지)

**대체 방법**: 로그 형식이 변경되면 OC-Memory가 정규식 패턴을 사용합니다 (`config.yaml`에서 구성 가능)

#### 9.1.3 Version Compatibility (버전 호환성)

**테스트된 버전**:
- OpenClaw v2026.2.1 ✅
- OpenClaw v2026.2.2 ✅
- OpenClaw v2026.2.3 ✅
- OpenClaw v2026.2.6 ✅

**호환성 변경 사항**: OpenClaw가 로그 형식 또는 파일 위치를 변경하는 경우 `config.yaml` 업데이트:
```yaml
logs:
  path: ~/.openclaw/logs/chat.log  # 위치가 변경되면 업데이트
  format: regex                     # regex | json
  user_pattern: '^\[.*?\] USER: (.+)$'
  assistant_pattern: '^\[.*?\] ASSISTANT: (.+)$'
```

### 9.2 LLM API Integration (LLM API 통합)

#### 9.2.1 Supported Providers (지원 공급자)

| 공급자 | 모델 예시 | API 베이스 |
|--------|-----------|------------|
| **OpenAI** | gpt-4o, gpt-5-mini | https://api.openai.com/v1 |
| **Anthropic** | claude-opus-4-6, claude-sonnet-4-5 | https://api.anthropic.com/v1 |
| **Google** | gemini-2.5-flash | https://generativelanguage.googleapis.com/v1 |
| **OpenRouter** | google/gemini-2.5-flash | https://openrouter.ai/api/v1 |

**구성**:
```yaml
observer:
  model: google/gemini-2.5-flash
  api_key: ${GOOGLE_API_KEY}
  api_base: https://openrouter.ai/api/v1  # 선택사항
  temperature: 0.3
  max_tokens: 2000

reflector:
  model: google/gemini-2.5-flash
  api_key: ${GOOGLE_API_KEY}
  temperature: 0.0
  max_tokens: 5000
```

#### 9.2.2 API Requirements (API 요구사항)

**관찰자 에이전트**:
- 입력: 대화 로그 (최대 50개 메시지)
- 출력: 구조화된 관찰 (JSON 또는 마크다운)
- 토큰 예산: ~2,000 출력 토큰
- 지연: <5초

**반영자 에이전트**:
- 입력: 관찰 배치 (최대 100개)
- 출력: 압축된 요약
- 토큰 예산: ~5,000 출력 토큰
- 지연: <30초

**대체 전략**:
- 기본: Google Gemini 2.5 Flash (빠르고 저렴)
- 보조: OpenAI GPT-4o (기본 실패 시)
- 3차: Claude Sonnet 4.5 (둘 다 실패 시)

### 9.3 LLM 모델 요구사항 (LLM Model Requirements)

#### 9.3.1 최소 요구사항 (Minimum Requirements)

Observer와 Reflector가 정상 작동하기 위한 LLM 최소 요구사항:

| 요구사항 | 최소 수준 | 검증 방법 |
|----------|----------|----------|
| **텍스트 요약 능력** | 중급 | 10,000 토큰 → 2,000 토큰 압축 가능 |
| **구조화 출력** | 중급 | Markdown 포맷 생성 능력 |
| **지시 따르기** | 중급 | System Prompt 준수 |
| **일관성** | 중급 | 동일 입력 → 유사 출력 (temperature 0.0-0.3) |
| **컨텍스트 윈도우** | 최소 32K | 30,000 토큰 입력 처리 가능 |

**부적합 모델**:
- ❌ GPT-3.5-turbo-instruct (16K 윈도우, 부족)
- ❌ 로컬 소형 모델 (Llama 7B 등, 구조화 능력 부족)

#### 9.3.2 추천 모델 (Recommended Models)

##### 🥇 1순위: Google Gemini 2.5 Flash

**기술 사양**:
- 컨텍스트 윈도우: 1M 토큰
- 출력 토큰: 최대 8K
- 응답 속도: ~500ms
- 가격: $0.075/1M 입력 토큰

**검증 결과**:
- ✅ Mastra 벤치마크: 94.87% LongMemEval
- ✅ 압축률: 5-40배 달성
- ✅ 프로덕션 검증됨

**구성 예시**:
```yaml
observer:
  model: google/gemini-2.5-flash
  temperature: 0.3
  max_output_tokens: 2000

reflector:
  model: google/gemini-2.5-flash
  temperature: 0.0
  max_output_tokens: 5000
```

##### 🥈 2순위: OpenAI GPT-4o-mini

**기술 사양**:
- 컨텍스트 윈도우: 128K 토큰
- 출력 토큰: 최대 16K
- 응답 속도: ~800ms
- 가격: $0.15/1M 입력 토큰

**장점**:
- ✅ GPT-4 계열 품질
- ✅ 기존 OpenAI 키 재사용
- ✅ 안정적인 API

**구성 예시**:
```yaml
observer:
  model: gpt-4o-mini
  api_base: https://api.openai.com/v1

reflector:
  model: gpt-4o-mini
```

##### 🥉 3순위: Claude 3 Haiku

**기술 사양**:
- 컨텍스트 윈도우: 200K 토큰
- 출력 토큰: 최대 4K
- 응답 속도: ~600ms
- 가격: $0.25/1M 입력 토큰

#### 9.3.3 모델 선택 기준 (Selection Criteria)

**비용 우선 (Cost-First)**:
```
Gemini 2.5 Flash → GPT-4o-mini → Claude 3 Haiku
```

**품질 우선 (Quality-First)**:
```
GPT-4o-mini → Claude 3 Haiku → Gemini 2.5 Flash
```

**프라이버시 우선 (Privacy-First)**:
```
Claude 3 Haiku → GPT-4o-mini → Gemini 2.5 Flash
```

**기존 인프라 활용**:
```
이미 OpenAI 계약 → GPT-4o-mini
이미 Anthropic 계약 → Claude 3 Haiku
새로 시작 → Gemini 2.5 Flash
```

#### 9.3.4 성능 요구사항 (Performance Requirements)

| 메트릭 | 목표값 | 검증 모델 |
|--------|--------|----------|
| **압축률** | 5-10배 | Gemini 2.5 Flash: ✅ 달성 |
| **정확도** | LongMemEval 85%+ | Gemini 2.5 Flash: 94.87% |
| **응답 시간** | <2초 (Observer) | 모든 추천 모델: ✅ |
| **응답 시간** | <3초 (Reflector) | 모든 추천 모델: ✅ |
| **토큰 효율** | 출력 2K (Observer) | 모든 추천 모델: ✅ |

#### 9.3.5 비용 분석 (Cost Analysis)

**시나리오: 월 50회 대화/일, 10,000 토큰/대화**

| 모델 | 월간 비용 | 연간 비용 | vs Gemini |
|------|----------|----------|----------|
| **Gemini 2.5 Flash** | $0.07 | $0.84 | 기준 |
| **GPT-4o-mini** | $0.13 | $1.56 | +86% |
| **Claude 3 Haiku** | $0.22 | $2.64 | +214% |
| **GPT-4o (비추천)** | $4.38 | $52.56 | +6157% |

**결론**: Gemini 2.5 Flash가 비용 대비 성능 최고

#### 9.3.6 Acceptance Criteria (승인 기준)

선택한 LLM 모델은 다음 기준을 충족해야 합니다:

- [ ] 30,000 토큰 입력 처리 가능 (컨텍스트 윈도우)
- [ ] 2,000 토큰 이상 출력 가능
- [ ] LongMemEval 85% 이상 또는 동등 성능
- [ ] 5배 이상 압축률 달성
- [ ] 응답 시간 3초 이내
- [ ] 월 API 비용 $1 이하 (권장)

### 9.4 ChromaDB Integration (ChromaDB 통합)

#### 9.4.1 Installation (설치)

```bash
pip install chromadb
```

**버전 요구사항**: chromadb >= 0.4.0

#### 9.4.2 Configuration (구성)

```yaml
memory:
  hot:
    storage: chromadb
    db_path: ~/.openclaw/memory_db
    embedding_model: text-embedding-3-small  # OpenAI 모델
    similarity_metric: cosine
```

#### 9.4.3 Embedding Generation (임베딩 생성)

**기본값**: OpenAI `text-embedding-3-small` 사용
- 차원: 1536
- 비용: 1K 토큰당 $0.00002
- 속도: 임베딩당 ~100ms

**대안**: ChromaDB 기본 임베딩
- 차원: 384
- 비용: 무료 (로컬 모델)
- 속도: 임베딩당 ~50ms

### 9.5 Obsidian Integration (Obsidian 통합)

#### 9.5.1 Obsidian CLI (Yakitrak)

**설치**:
```bash
# macOS
brew tap yakitrak/yakitrak
brew install yakitrak/yakitrak/obsidian-cli

# Linux
curl -L https://github.com/Yakitrak/obsidian-cli/releases/latest/download/obsidian-cli-linux -o /usr/local/bin/obsidian-cli
chmod +x /usr/local/bin/obsidian-cli

# Windows
# https://github.com/Yakitrak/obsidian-cli/releases에서 다운로드
```

**구성**:
```bash
# 기본 볼트 설정
obsidian-cli set-default "Main"
```

**필수 작업**:
```bash
# 노트 생성
obsidian-cli create "OC-Memory/2026-02-12.md" --content "..."

# 노트 검색
obsidian-cli search-content "API 인증"

# 노트 읽기
obsidian-cli print "OC-Memory/2026-02-12.md"
```

#### 9.5.2 Obsidian REST API (Optional) (Obsidian REST API - 선택사항)

**플러그인**: Obsidian용 Local REST API

**설치**:
1. Obsidian 커뮤니티 플러그인에서 플러그인 설치
2. 설정 → 커뮤니티 플러그인에서 활성화
3. 플러그인 설정에서 API 키 구성
4. 기본 포트: 27124

**구성**:
```yaml
obsidian_rest:
  enabled: true
  api_key: ${OBSIDIAN_API_KEY}
  host: https://127.0.0.1:27124
  verify_ssl: false  # 자체 서명 인증서
```

### 9.6 Dropbox Integration (Dropbox 통합)

#### 9.6.1 Dropbox SDK

**설치**:
```bash
pip install dropbox
```

**구성**:
```yaml
dropbox:
  enabled: true
  access_token: ${DROPBOX_ACCESS_TOKEN}
  vault_path: /Apps/Obsidian/Main
  sync_interval: 3600  # 초 (1시간)
```

#### 9.6.2 OAuth Setup (OAuth 설정)

**단계**:
1. https://www.dropbox.com/developers/apps에서 Dropbox 앱 생성
2. 액세스 토큰 생성
3. 환경 변수에 토큰 설정:
   ```bash
   export DROPBOX_ACCESS_TOKEN="your_token_here"
   ```
4. 연결 테스트:
   ```bash
   oc-memory config validate
   ```

**필수 권한**:
- `files.metadata.read`
- `files.content.read`
- `files.content.write`

### 9.7 TUI Dependencies (TUI 의존성)

#### 9.7.1 Questionary Library (Questionary 라이브러리)

**설명**: 설정 마법사를 구축하기 위한 대화형 명령줄 사용자 인터페이스 라이브러리입니다.

**설치**:
```bash
pip install questionary
```

**버전 요구사항**: questionary >= 2.0.0

**설정 마법사에서 사용**:
- 검증이 있는 텍스트 입력 프롬프트
- 키보드 네비게이션이 있는 선택 메뉴
- 확인 프롬프트 (예/아니오)
- 마스킹이 있는 비밀번호 입력
- 진행 표시기
- 색상 출력 형식

**사용된 기능**:
```python
import questionary

# 텍스트 입력
answer = questionary.text("설치 디렉토리:", default="~/.openclaw/oc-memory").ask()

# 선택 메뉴
provider = questionary.select(
    "공급자 선택:",
    choices=[
        "Google Gemini (권장 - 빠르고 저렴)",
        "OpenAI GPT",
        "Anthropic Claude",
        "OpenRouter (다중 공급자)"
    ]
).ask()

# 비밀번호 입력
api_key = questionary.password("Google API 키:").ask()

# 확인
proceed = questionary.confirm("설치를 진행하시겠습니까?").ask()
```

**구성**:
```yaml
setup:
  tui_library: questionary
  color_scheme: default  # default | monokai | vim
  show_defaults: true
  validate_inputs: true
```

**의존성**:
- `prompt_toolkit` >= 3.0 (questionary와 함께 자동 설치)
- Windows ANSI 색상 지원 (최신 Windows 10+에 포함)

---

## 10. Acceptance Criteria (승인 기준)

### 10.1 P0 Feature Acceptance (P0 기능 승인)

#### AC-P0-001: Log Monitoring (로그 모니터링)

**조건**: OpenClaw가 `chat.log`에 대화를 적극적으로 로깅하고 있음
**실행**: OC-Memory 데몬이 실행 중
**결과**:
- ✅ 새 로그 항목이 1초 이내에 감지됨
- ✅ 로그 항목 누락 없음 (100% 캡처율)
- ✅ 유휴 모니터링 중 CPU 사용량이 5% 미만으로 유지됨
- ✅ 데이터 손실 없이 로그 파일 로테이션에서 데몬 생존

**테스트 시나리오**:
```
1. OC-Memory 데몬 시작
2. OpenClaw에서 100개의 테스트 대화 생성
3. 100개의 대화가 모두 감지되었는지 확인
4. 로그 파일 로테이션 (mv chat.log chat.log.1)
5. 50개의 대화 추가 생성
6. 50개의 새 대화가 모두 감지되었는지 확인
```

#### AC-P0-002: Observation Generation (관찰 생성)

**조건**: 새 대화 로그가 감지됨
**실행**: 관찰자 에이전트가 로그 처리
**결과**:
- ✅ 5초 이내에 관찰 생성
- ✅ 우선순위 분류 정확도 ≥85% (수동 레이블과 비교 검증)
- ✅ 모든 관찰에 유효한 타임스탬프 포함
- ✅ 고유 ID 충돌률 <0.001%
- ✅ ChromaDB에 관찰이 성공적으로 저장됨

**테스트 시나리오**:
```
1. 관찰자에게 50개의 테스트 대화 제공
2. 예상 우선순위 수동 레이블링 (높음/중간/낮음)
3. 관찰자 출력과 수동 레이블 비교
4. 정확도 계산: (올바른 분류 / 전체) ≥ 0.85
5. 모든 관찰에 타임스탬프와 고유 ID가 있는지 확인
```

#### AC-P0-003: Active Memory File (활성 메모리 파일)

**조건**: 관찰이 핫 메모리에 저장됨
**실행**: 활성 메모리 파일이 업데이트됨
**결과**:
- ✅ 파일 업데이트가 2초 이내에 완료
- ✅ 토큰 수가 구성된 제한(기본값 4000) 미만으로 유지
- ✅ 파일이 유효한 마크다운 형식 (마크다운 파서로 검증)
- ✅ 파일 내용이 최근 관찰을 정확하게 반영
- ✅ OpenClaw가 파일을 성공적으로 읽을 수 있음

**테스트 시나리오**:
```
1. 200개의 관찰 생성 (토큰 제한 초과)
2. active_memory.md 업데이트 트리거
3. 파일 크기가 4000 토큰 미만인지 확인
4. 마크다운 파서로 파일 파싱 (오류 없음)
5. OpenClaw가 파일을 읽도록 구성
6. OpenClaw가 응답에서 메모리를 참조하는지 확인
```

#### AC-P0-004: OpenClaw Integration (OpenClaw 통합)

**조건**: OC-Memory-Sidecar가 설치되고 구성됨
**실행**: 사용자가 OpenClaw 시스템 프롬프트에 메모리 지침 추가
**결과**:
- ✅ OpenClaw가 active_memory.md를 성공적으로 읽음
- ✅ OpenClaw가 응답에서 메모리 콘텐츠를 참조
- ✅ OpenClaw 핵심 코드 수정 불필요
- ✅ OpenClaw v2026.2.x 버전 전반에 걸쳐 통합 작동

**테스트 시나리오**:
```
1. OC-Memory-Sidecar 설치
2. OpenClaw 시스템 프롬프트에 메모리 지침 추가
3. 과거 컨텍스트를 참조하는 대화 생성
4. OpenClaw에게 질문: "내가 언급한 선호도는 무엇인가요?"
5. OpenClaw 응답에 저장된 선호도가 포함되어 있는지 확인
6. OpenClaw 설치 디렉토리 검사
7. 핵심 파일이 수정되지 않았는지 확인 (git diff = 빈 결과)
```

### 10.2 P1 Feature Acceptance (P1 기능 승인)

#### AC-P1-001: Semantic Search (시맨틱 검색)

**조건**: ChromaDB에 10,000개의 관찰 저장됨
**실행**: 사용자가 자연어 쿼리로 검색
**결과**:
- ✅ 검색 결과가 500ms 이내에 반환
- ✅ 상위 3개 결과의 관련성 점수 ≥0.75
- ✅ 결과가 유사도로 순위 지정됨 (가장 높음 우선)
- ✅ 검색 정확도 ≥85% (LongMemEval 하위 집합에서 검증)

**테스트 시나리오**:
```
1. 10,000개의 테스트 관찰 저장 (레이블링된 데이터셋)
2. 100개의 테스트 쿼리 실행
3. 각 쿼리의 응답 시간 측정
4. P@3 (Precision at 3) 계산: 쿼리당 (상위 3개 중 관련 / 3)
5. 100개 쿼리에 걸쳐 평균 P@3 ≥ 0.85
```

#### AC-P1-002: TTL Management (TTL 관리)

**조건**: 핫 메모리에 다양한 연령의 관찰 존재
**실행**: 일일 TTL 작업 실행
**결과**:
- ✅ 90일보다 오래된 관찰이 웜 메모리로 이동
- ✅ 핫 메모리 크기가 예상만큼 감소
- ✅ 전환 중 관찰 손실 없음
- ✅ 웜 메모리 마크다운 파일이 올바르게 생성됨
- ✅ 프로세스가 10분 이내에 완료

**테스트 시나리오**:
```
1. 핫 메모리를 관찰로 채우기:
   - 1000개 관찰 (0-89일) → 핫에 유지
   - 500개 관찰 (90일 이상) → 웜으로 이동
2. TTL 작업 실행: oc-memory archive --older-than 90 --to warm
3. 핫 메모리 개수가 ~500 감소했는지 확인
4. 웜 메모리 마크다운 파일이 생성되었는지 확인
5. 웜 파일에 500개 관찰이 모두 존재하는지 확인
6. 실행 시간 < 10분 측정
```

#### AC-P1-003: 3-Tier Architecture (3계층 아키텍처)

**조건**: 다양한 연령 범위에 걸친 관찰 존재
**실행**: 시스템이 400일 동안 작동
**결과**:
- ✅ 핫 메모리에 0-90일 관찰만 포함
- ✅ 웜 메모리에 90-365일 관찰만 포함
- ✅ 콜드 메모리에 365일 이상 관찰만 포함
- ✅ 액세스 지연이 계층 사양과 일치:
  - 핫: <100ms
  - 웜: <2초
  - 콜드: 수동 (사용자 시작)
- ✅ 계층 전환 간 데이터 손실 없음

**테스트 시나리오**:
```
1. 400일의 관찰 시뮬레이션 (역날짜)
2. TTL 작업을 실행하여 모든 계층 채우기
3. 각 계층에서 관찰 쿼리, 지연 측정
4. 계층별 지연 목표 충족 확인
5. 모든 관찰 감사: 중복/손실 없는지 확인
```

### 10.3 P2 Feature Acceptance (P2 기능 승인)

#### AC-P2-001: Obsidian Integration (Obsidian 통합)

**조건**: Obsidian CLI가 설치되고 구성됨
**실행**: 관찰이 콜드 메모리로 아카이브됨
**결과**:
- ✅ Obsidian 볼트에 마크다운 파일 생성
- ✅ Obsidian에서 파일 읽기 가능 (형식 오류 없음)
- ✅ 프론트매터 메타데이터 올바름 (날짜, 태그, 유형)
- ✅ 그래프 뷰가 노트 간 연결 표시
- ✅ Obsidian 검색으로 아카이브된 관찰 찾기

**테스트 시나리오**:
```
1. ~/Documents/Obsidian/Main에 Obsidian 볼트 구성
2. 100개 관찰을 콜드 메모리로 아카이브
3. Obsidian 열기, OC-Memory 폴더로 이동
4. 100개 마크다운 파일이 생성되었는지 확인
5. 임의의 파일 열기, 프론트매터 존재 확인
6. 그래프 뷰 열기, 노트 표시 확인
7. 키워드 검색, 결과 반환 확인
```

#### AC-P2-002: Dropbox Sync (Dropbox 동기화)

**조건**: Dropbox 통합 활성화
**실행**: 관찰이 콜드 메모리로 아카이브됨
**결과**:
- ✅ 5분 이내에 Dropbox에 파일 업로드
- ✅ 업로드 중 데이터 손실 없음
- ✅ 충돌 해결이 올바르게 작동
- ✅ 역 쿼리가 Dropbox에서 파일 검색
- ✅ 오프라인 모드가 나중에 업로드를 대기열에 추가

**테스트 시나리오**:
```
1. 설정에서 Dropbox 동기화 활성화
2. 50개 관찰을 콜드 메모리로 아카이브
3. 최대 5분 대기, Dropbox 웹 인터페이스 확인
4. Dropbox에 50개 파일이 모두 존재하는지 확인
5. 인터넷 연결 해제, 10개 관찰 추가 아카이브
6. 인터넷 재연결, 10개 파일이 업로드되었는지 확인
7. Dropbox에서 관찰 쿼리: oc-memory search "test" --source dropbox
8. Dropbox에서 결과가 반환되는지 확인
```

### 10.4 Non-Functional Acceptance (비기능 승인)

#### AC-NF-001: Performance (성능)

**요구사항**: 시스템이 부하 상태에서 목표 성능 메트릭 유지

**테스트 시나리오**:
```
부하 테스트:
- 하루에 500개 대화
- 대화당 50개 관찰
- 하루에 25,000개 관찰
- 7일 동안 실행 (175,000개 관찰)

검증할 메트릭:
- ✅ 로그 감지 지연: ≤1초 (99번째 백분위수)
- ✅ 관찰 생성: ≤5초 (99번째 백분위수)
- ✅ 검색 응답: ≤500ms (99번째 백분위수)
- ✅ 메모리 파일 업데이트: ≤2초 (99번째 백분위수)
- ✅ CPU 사용량 (유휴): ≤5%
- ✅ CPU 사용량 (활성): ≤30%
- ✅ RAM 사용량: ≤512 MB
```

#### AC-NF-002: Reliability (신뢰성)

**요구사항**: 시스템이 장애에서 우아하게 복구

**테스트 시나리오**:
```
크래시 복구:
1. OC-Memory 데몬 시작
2. 100개 관찰 생성
3. 데몬 강제 종료 (kill -9 [PID])
4. 데몬 재시작
5. 확인:
   - ✅ 100개 관찰이 모두 여전히 존재
   - ✅ 데몬이 마지막 상태에서 재개
   - ✅ 중복 관찰 생성 안 됨

로그 로테이션:
1. 10,000개 로그 라인 생성
2. 로그 파일 로테이션 (logrotate 시뮬레이션)
3. 10,000개 라인 추가 생성
4. 확인:
   - ✅ 20,000개 라인이 모두 처리됨
   - ✅ 로테이션 경계에서 라인 누락 없음

API 장애:
1. 잘못된 API 키로 관찰자 구성
2. 대화 생성
3. 확인:
   - ✅ 오류가 명확하게 로깅됨
   - ✅ 데몬이 계속 실행됨
   - ✅ 백오프로 재시도
   - ✅ 구성된 경우 보조 모델로 대체
```

#### AC-NF-003: Security (보안)

**요구사항**: 민감한 데이터가 보호됨

**테스트 시나리오**:
```
파일 권한:
1. OC-Memory-Sidecar 설치
2. 파일 권한 확인:
   - ✅ config.yaml: 600 (소유자만)
   - ✅ .env: 600 (소유자만)
   - ✅ memory_db/: 700 (소유자만)
   - ✅ active_memory.md: 644 (소유자 쓰기, 그룹/기타 읽기)

민감 데이터 마스킹:
1. 대화에 API 키 붙여넣기: "내 키는 sk-1234567890abcdef입니다"
2. active_memory.md 확인
3. 검증:
   - ✅ API 키 마스킹됨: "내 키는 sk-***************입니다"
   - ✅ 원본 대화 로그는 그대로
   - ✅ 저장 전에 마스킹 적용됨

네트워크 전송:
1. 작동 중 네트워크 트래픽 모니터링
2. 확인:
   - ✅ 외부 서버로 데이터 전송 없음 (LLM API 제외)
   - ✅ LLM API 호출이 HTTPS 사용
   - ✅ 원격 측정 또는 분석 전송 없음
```

---

## 11. Milestones & Timeline (마일스톤 및 타임라인)

### 11.1 Development Phases (개발 단계)

#### Phase 1: MVP Foundation (1-2주차)

**목표**: 기본 메모리 작동을 위한 핵심 기능

**제공 결과물**:
- ✅ 로그 모니터링 시스템 (LogWatcher)
- ✅ 관찰자 에이전트 통합
- ✅ 기본 관찰 저장소 (인메모리)
- ✅ 활성 메모리 파일 생성
- ✅ CLI 골격 (시작/중지/상태)
- ✅ 구성 시스템 (YAML)
- ✅ 기본 문서 (README, 설정 가이드)

**성공 기준**:
- OpenClaw 로그 모니터링 가능
- 대화에서 관찰 생성 가능
- active_memory.md 파일 생성 가능
- OpenClaw가 메모리 파일 읽고 사용 가능

**필요 리소스**:
- 1명의 백엔드 개발자 (풀타임)
- LLM API 액세스 (Gemini 2.5 Flash)

**위험 요소**:
- OpenClaw의 로그 형식 변경 → 완화: 구성 가능한 정규식 패턴
- LLM API 불안정 → 완화: 다중 공급자 지원

#### Phase 2: Persistence & Search (3-4주차)

**목표**: 영구 저장소 및 시맨틱 검색을 위한 ChromaDB 추가

**제공 결과물**:
- ✅ ChromaDB 통합
- ✅ 임베딩을 사용한 관찰 저장
- ✅ 시맨틱 검색 API
- ✅ CLI 검색 명령
- ✅ 압축을 위한 반영자 에이전트
- ✅ 토큰 카운팅 및 예산 관리
- ✅ 단위 테스트 (80%+ 커버리지)

**성공 기준**:
- 재시작 간 관찰 지속
- 시맨틱 검색이 관련 결과 반환 (85%+ 정확도)
- 압축이 최소 5:1 비율 달성
- 모든 CLI 명령 작동

**필요 리소스**:
- 1명의 백엔드 개발자 (풀타임)
- 1명의 QA 엔지니어 (파트타임)

**위험 요소**:
- 대규모 ChromaDB 성능 → 완화: 100K 관찰로 벤치마크
- 임베딩 비용 → 완화: OpenAI text-embedding-3-small 사용 (저렴)

#### Phase 3: TTL & 3-Tier Architecture (5-6주차)

**목표**: 계층적 메모리 관리 구현

**제공 결과물**:
- ✅ TTL 매니저 구현
- ✅ 핫 → 웜 아카이빙 (자동)
- ✅ 웜 → 콜드 아카이빙 (수동 승인)
- ✅ 마크다운 아카이브 파일 생성
- ✅ CLI 아카이브 명령
- ✅ 예약된 백그라운드 작업 (크론 유사)
- ✅ 아카이브 통계 및 보고

**성공 기준**:
- 오래된 관찰이 자동으로 아카이브됨
- 핫 메모리가 100MB 미만 유지
- 아카이빙 프로세스가 10분 미만에 완료
- 전환 중 데이터 손실 없음

**필요 리소스**:
- 1명의 백엔드 개발자 (풀타임)

**위험 요소**:
- 대규모 아카이빙 성능 → 완화: 배치 처리
- 사용자 승인 UX 불명확 → 완화: 명확한 CLI 프롬프트

#### Phase 4: Obsidian & Cloud Integration (7-8주차)

**목표**: Obsidian 및 Dropbox를 사용한 콜드 메모리 추가

**제공 결과물**:
- ✅ Obsidian CLI 통합
- ✅ Dropbox API 통합
- ✅ 역 쿼리 (Dropbox → 로컬)
- ✅ Obsidian REST API 지원 (선택사항)
- ✅ 클라우드 동기화 모니터링
- ✅ 충돌 해결 로직
- ✅ 통합 테스트

**성공 기준**:
- Obsidian 볼트에 아카이브 생성
- Dropbox 동기화가 안정적으로 작동
- 역 쿼리가 클라우드 데이터 검색
- Obsidian 그래프 뷰가 메모리 연결 표시

**필요 리소스**:
- 1명의 백엔드 개발자 (풀타임)
- Obsidian CLI (오픈소스)
- Dropbox API 액세스 (무료 티어)

**위험 요소**:
- Dropbox API 속도 제한 → 완화: 백오프/재시도 구현
- Obsidian CLI 호환성 변경 → 완화: CLI 버전 고정, 릴리스 모니터링

#### Phase 5: Polish & Optimization (9-10주차)

**목표**: 성능 튜닝, 문서화 및 출시 준비

**제공 결과물**:
- ✅ 성능 최적화 (프로파일링, 캐싱)
- ✅ 포괄적인 문서 (API, CLI, 통합)
- ✅ 사용자 가이드 및 튜토리얼
- ✅ 예제 구성
- ✅ 벤치마크 스위트 (LongMemEval 하위 집합)
- ✅ CI/CD 파이프라인
- ✅ 릴리스 패키징

**성공 기준**:
- 모든 승인 기준 충족
- 문서 완료 및 명확
- LongMemEval에서 벤치마크 점수 ≥85%
- 공개 릴리스 준비 완료

**필요 리소스**:
- 1명의 백엔드 개발자 (풀타임)
- 1명의 기술 작가 (파트타임)

**위험 요소**:
- 목표 이하 벤치마크 점수 → 완화: 프롬프트 조정, 압축 비율
- 문서 미완성 → 완화: 1단계에서 문서 조기 시작

### 11.2 Timeline Summary (타임라인 요약)

```
1-2주차:  1단계 - MVP 기반
3-4주차:  2단계 - 지속성 및 검색
5-6주차:  3단계 - TTL 및 3계층 아키텍처
7-8주차:  4단계 - Obsidian 및 클라우드 통합
9-10주차: 5단계 - 다듬기 및 최적화
────────────────────────────────────────────────
총: 10주 (~2.5개월)
```

### 11.3 Milestone Checklist (마일스톤 체크리스트)

#### M1: MVP Ready (2주차 종료)

- [ ] 로그 모니터링 작동 (1초 지연)
- [ ] 관찰자가 관찰 생성
- [ ] 활성 메모리 파일 생성
- [ ] OpenClaw 통합 테스트
- [ ] CLI 시작/중지/상태 작동
- [ ] 기본 문서 완료

#### M2: Core Features Complete (4주차 종료)

- [ ] ChromaDB 저장소 작동
- [ ] 시맨틱 검색 기능 (85%+ 정확도)
- [ ] 반영자 압축 작동 (5:1 비율)
- [ ] CLI 검색 명령 작동
- [ ] 단위 테스트 통과 (80%+ 커버리지)
- [ ] 성능 목표 충족 (P0 기능)

#### M3: Memory Management Complete (6주차 종료)

- [ ] TTL 매니저 구현
- [ ] 핫 → 웜 아카이빙 자동
- [ ] 승인을 통한 웜 → 콜드 아카이빙
- [ ] 아카이브 명령 작동
- [ ] 예약된 작업 실행
- [ ] 테스트에서 데이터 손실 없음

#### M4: Cloud Integration Complete (8주차 종료)

- [ ] Obsidian CLI 통합 작동
- [ ] Dropbox 동기화 기능
- [ ] 역 쿼리 작동
- [ ] 충돌 해결 테스트
- [ ] 통합 테스트 통과
- [ ] 클라우드 동기화 안정

#### M5: Production Ready (10주차 종료)

- [ ] 모든 승인 기준 충족
- [ ] 문서 완료
- [ ] 벤치마크 점수 ≥85%
- [ ] 성능 최적화
- [ ] CI/CD 파이프라인 작동
- [ ] 릴리스 패키지 준비

### 11.4 Post-Launch Roadmap (출시 후 로드맵)

#### Phase 6: Community Feedback (3-4개월차)

**목표**:
- 사용자 피드백 수집
- 버그 및 사용성 문제 수정
- 질문에 기반한 문서 개선
- 가장 요청된 기능 추가

**잠재적 기능** (예상 피드백 기반):
- 웹 대시보드 (P2)
- 다중 에이전트 메모리 공유 (P2)
- 고급 분석 (P2)
- 가져오기/내보내기 도구
- 메모리 시각화 도구

#### Phase 7: Advanced Features (5-6개월차)

**목표**:
- 수요에 따라 P2 기능 구현
- 지식 그래프 통합 탐색 (Zep/Graphiti)
- 메모리 인사이트 및 권장사항 추가
- 매우 큰 데이터셋 최적화 (1M+ 관찰)

**연구 영역**:
- 하이브리드 메모리 시스템 (관찰 + 지식 그래프)
- 자동화된 프롬프트 최적화
- 다중 모달 메모리 (이미지, 코드, 문서)
- 연합 메모리 (다중 디바이스 동기화)

---

## Appendix A: Terminology (부록 A: 용어)

| 용어 | 정의 |
|------|------|
| **관찰 (Observation)** | 대화 로그에서 추출된 구조화된 정보 조각 |
| **반영 (Reflection)** | 여러 관찰의 압축된 요약 |
| **핫 메모리 (Hot Memory)** | 빠른 액세스를 위해 ChromaDB에 저장된 가장 최근 관찰 (0-90일) |
| **웜 메모리 (Warm Memory)** | 마크다운 파일에 저장된 아카이브된 관찰 (90-365일) |
| **콜드 메모리 (Cold Memory)** | Obsidian 볼트에 저장된 장기 관찰 (365일 이상) |
| **TTL (Time-To-Live)** | 관찰이 다음 계층으로 아카이브되기 전의 기간 |
| **관찰자 에이전트 (Observer Agent)** | 로그에서 관찰을 추출하는 LLM 기반 에이전트 |
| **반영자 에이전트 (Reflector Agent)** | 관찰을 압축하는 LLM 기반 에이전트 |
| **사이드카 (Sidecar)** | 컴포넌트가 메인 시스템과 별도로 함께 실행되는 아키텍처 패턴 |
| **제로 핵심 수정 (Zero-Core-Modification)** | OpenClaw의 핵심 코드를 수정하지 않는 원칙 |
| **시맨틱 검색 (Semantic Search)** | 벡터 임베딩 및 유사도를 사용한 의미 기반 검색 |

## Appendix B: References (부록 B: 참고 문헌)

### Internal Documents (내부 문서)
- [OC_Memory_Improvement_Strategy.md](./OC_Memory_Improvement_Strategy.md)
- [OC_Memory_Tech_Spec.md](./OC_Memory_Tech_Spec.md)
- [OC_Memory_Comparative_Analysis.md](./OC_Memory_Comparative_Analysis.md)

### External Resources (외부 리소스)

**OpenClaw**:
- [OpenClaw v2026.2.6 릴리스 노트](https://cybersecuritynews.com/openclaw-v2026-2-6-released/)
- [OpenClaw GitHub 릴리스](https://github.com/openclaw/openclaw/releases)

**Mastra 관찰 메모리**:
- [Mastra OM 발표](https://mastra.ai/blog/observational-memory)
- [Mastra 문서](https://mastra.ai/docs/memory/observational-memory)
- [Mastra 변경 로그 2026-02-04](https://mastra.ai/blog/changelog-2026-02-04)
- [VentureBeat: 관찰 메모리가 비용 10배 절감](https://venturebeat.com/data/observational-memory-cuts-ai-agent-costs-10x-and-outscores-rag-on-long)

**연구**:
- [Mastra 연구: LongMemEval에서 95%](https://mastra.ai/research/observational-memory)
- [CoALA 논문: 인지 아키텍처](https://arxiv.org/abs/2309.02427)

**도구**:
- [ChromaDB 문서](https://docs.trychroma.com/)
- [Obsidian CLI (Yakitrak)](https://github.com/Yakitrak/obsidian-cli)
- [Dropbox API 문서](https://www.dropbox.com/developers/documentation)

## Appendix C: Change Log (부록 C: 변경 로그)

| 버전 | 날짜 | 변경 사항 | 작성자 |
|------|------|-----------|--------|
| 2.0 | 2026-02-12 | 향상된 구조, 2026 업데이트, 상세한 사용자 스토리 및 포괄적인 승인 기준으로 완전히 재작성 | Argo |
| 1.0 | 2026-02-12 | 연구를 기반으로 한 초안 | Argo |

---

**문서 상태**: 초안
**다음 검토**: 2026-02-19
**승인자**: Argo (PM), Asin (기술 리더)

---

**문서 끝**
