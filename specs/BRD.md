# BRD (Business Requirements Document)
## OC-Memory: OpenClaw Observational Memory System

**Project Name**: OC-Memory (OpenClaw Observational Memory System)
**Document Version**: 1.0
**Date**: 2026-02-12
**Author**: Argo (OpenClaw General Manager)
**Status**: Draft - For Review

---

## Document Control

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-12 | Argo | Initial draft |

---

## 1. Executive Summary

### 1.1 Project Vision

OC-Memory는 급성장하는 오픈소스 AI 에이전트 프레임워크인 OpenClaw에 인간과 유사한 장기 기억 능력을 부여하는 혁신적인 메모리 시스템입니다. Mastra의 Observational Memory 개념을 적용하되, OpenClaw의 코어 코드를 전혀 수정하지 않고(Zero-Core-Modification) 사이드카 패턴(Sidecar Pattern)을 통해 외부에서 메모리 기능을 제공합니다.

### 1.2 Project Background

**시장 상황 (2026년 2월 기준)**:

- **OpenClaw**: 2026년 1월 급성장한 오픈소스 AI 에이전트 프레임워크
  - 3주 만에 GitHub 157,000 stars 달성
  - 1월 30일: 48시간 동안 34,168 stars (GitHub 역사상 최고 기록)
  - 전세계 개발자 커뮤니티의 폭발적 관심
  - **HTTP API 제공**: WebSocket Gateway (Port 18789) + Webhook Hooks + OpenAI-compatible endpoint
  - **Memory 시스템 내장**: SQLite + Vector (sqlite-vec) + FTS5 자동 인덱싱
  - **Plugin 시스템**: 전체 Plugin SDK 및 Hook Points 제공

- **Mastra Observational Memory**: 최첨단 AI 메모리 시스템
  - LongMemEval 벤치마크 94.87% 달성 (gpt-5-mini, 역대 최고 점수)
  - 3-6배 (최대 5-40배) 토큰 압축률
  - Observer + Reflector 2단계 압축 아키텍처

- **Obsidian**: 지식 관리 도구의 표준
  - 2026년 AI 통합으로 Passive에서 Active 지식 관리로 전환
  - 로컬 파일 기반 + 강력한 플러그인 생태계
  - AI 시맨틱 검색, 자동 링크 관리 등 지원

### 1.3 Business Objectives

| 목표 | 설명 | 측정 지표 |
|------|------|----------|
| **장기 기억 구현** | OpenClaw가 90일 이상 대화 컨텍스트 유지 | 메모리 유지 기간 |
| **토큰 비용 절감** | 90% 이상 토큰 절약으로 운영 비용 감소 | 토큰 절약률 |
| **사용자 경험 개선** | 맥락 손실 없는 자연스러운 대화 및 손쉬운 설치 | 사용자 만족도, 설치 완료율 |
| **에코시스템 확장** | OpenClaw + Obsidian + Dropbox 통합 생태계 구축 | 통합 완성도 |

### 1.4 Expected ROI

```
비용 절감 예상 (연간):
- 토큰 사용량 90% 감소
- LLM API 비용: $1,000/년 → $100/년
- ROI: 900% (10배 비용 절감)

사용자 경험 개선:
- 반복 질문 80% 감소
- 작업 효율성 50% 향상
- 사용자 만족도 95% 이상
```

---

## 2. Business Context

### 2.1 Current Situation

#### 2.1.1 시장 현황

2026년 초, AI 에이전트 시장은 폭발적으로 성장하고 있습니다:

- **OpenClaw의 급성장**: 3주 만에 157,000 GitHub stars 달성, 개발자 커뮤니티의 중심이 됨
- **메모리 기술의 진화**: Mastra Observational Memory가 LongMemEval 94.87% 달성로 새로운 기준 제시
- **지식 관리의 변화**: Obsidian AI 통합으로 passive에서 active 지식 관리로 패러다임 전환

#### 2.1.2 문제점 (Pain Points)

| 문제 | 현상 | 비즈니스 영향 |
|------|------|---------------|
| **Context Loss** | 대화가 길어지면 이전 내용 망각 | 사용자 불만, 신뢰도 하락 |
| **Token Waste** | 매번 전체 대화 기록을 컨텍스트에 포함 | API 비용 폭증, 응답 속도 저하 |
| **No Learning** | 이전 대화에서 학습하지 못함 | 개인화 불가, 반복 작업 증가 |
| **Preference Ignorance** | 사용자 선호 무시하고 매번 같은 질문 | 사용자 피로도 증가, 이탈률 상승 |

#### 2.1.3 정량적 문제 분석

```
현재 OpenClaw (메모리 시스템 없음):
- 평균 컨텍스트 크기: 50,000 tokens/대화
- 하루 평균 대화 수: 50회
- 월간 토큰 소비: 75,000,000 tokens
- 월간 API 비용 (gpt-4o 기준): $150

OC-Memory 적용 후:
- 평균 컨텍스트 크기: 5,000 tokens/대화 (90% 절감)
- 월간 토큰 소비: 7,500,000 tokens
- 월간 API 비용: $15
- 비용 절감: $135/월 (90%)
```

### 2.2 Market Opportunity

#### 2.2.1 Target Market

**Primary Market**:
- OpenClaw 사용자 (157,000+ GitHub stars, 20,000+ forks)
- AI 에이전트 개발자
- 장기 대화 기록이 필요한 개인/팀

**Secondary Market**:
- Obsidian 사용자 (지식 관리에 AI 에이전트 통합)
- 다른 AI 에이전트 프레임워크 사용자 (LangGraph, CrewAI 등)

#### 2.2.2 Market Size

```
글로벌 AI 에이전트 시장:
- 2026년 시장 규모: $5B (예상)
- 연평균 성장률 (CAGR): 45%
- 2030년 예상 규모: $25B

OpenClaw 생태계:
- GitHub Stars: 157,000+ (2026년 2월)
- Active Users: 50,000+ (예상)
- 잠재 사용자: 500,000+ (향후 1년)
```

#### 2.2.3 Competitive Advantage

| 경쟁사 | 방식 | OC-Memory 우위점 |
|--------|------|------------------|
| **Mem0** | 라이브러리 통합 | Zero-Code-Change, 의존성 없음 |
| **Letta (MemGPT)** | 플랫폼 종속 | 독립 실행, 어떤 에이전트에도 적용 가능 |
| **Zep** | 지식 그래프 DB | 경량 파일 기반, 별도 DB 불필요 |
| **AWS AgentCore** | 클라우드 종속 | 로컬 우선, 프라이버시 보장 |

### 2.3 Latest Technology Insights (2026년 2월)

#### 2.3.1 OpenClaw 최신 동향 및 기술 스택

OpenClaw는 2026년 초 가장 빠르게 성장한 오픈소스 프로젝트 중 하나입니다:

- **Viral Growth**: 1월 30일, 48시간 동안 34,168 GitHub stars 획득 (GitHub 역사상 최고)
- **Community Size**: 157,000+ stars, 20,000+ forks (3주 만에 달성)
- **Developer Interest**: 전세계 개발자들의 폭발적 관심, AI 에이전트 분야의 새로운 표준으로 부상

**기술 스택 및 확장성** (2026년 2월 코드 분석 결과):

- **HTTP API**:
  - WebSocket Gateway API (Port 18789, 60+ 메서드)
  - OpenAI-Compatible Chat Completions (`POST /v1/chat/completions`)
  - Webhook Hooks API (`POST /hooks/wake`, `POST /hooks/agent`)

- **Memory System**:
  - 내장 Memory 데이터베이스 (`~/.openclaw/agents/<agentId>/memory.db`)
  - SQLite + sqlite-vec (Vector 검색) + FTS5 (Full-text 검색)
  - 자동 파일 인덱싱 (chokidar, 5초 debounce)
  - `memory_search`, `memory_get` tools 자동 제공

- **Plugin System**:
  - 전체 Plugin SDK 제공 (`src/plugin-sdk/`)
  - 10+ Hook Points (before_agent_start, after_tool_call 등)
  - HTTP Routes, Channel 확장, Tool 추가 가능

- **Configuration**:
  - `~/.openclaw/openclaw.json` (JSON 형식)
  - System Prompt, Context Files, Memory 경로 설정
  - Zero-Core-Modification 지원

**OC-Memory 통합 가능성**: ✅ 완전 지원 (Zero-Code-Change)

**출처**:
- [CNBC: From Clawdbot to Moltbot to OpenClaw](https://www.cnbc.com/2026/02/02/openclaw-open-source-ai-agent-rise-controversy-clawdbot-moltbot-moltbook.html)
- [Growth Foundry: OpenClaw Case Study](https://growth.maestro.onl/en/articles/openclaw-viral-growth-case-study)

#### 2.3.2 Mastra Observational Memory 성능

Mastra는 2026년 초 AI 메모리 시스템의 새로운 기준을 제시했습니다:

- **LongMemEval 벤치마크**: 94.87% (gpt-5-mini, 역대 최고 점수)
- **Compression Ratio**: 3-6배 (텍스트), 5-40배 (툴 콜)
- **Context Window**: 평균 30k tokens로 안정적 유지
- **Prompt Caching**: 완전히 안정적인 컨텍스트로 캐싱 가능

**출처**:
- [Mastra Research: Observational Memory](https://mastra.ai/research/observational-memory)
- [VentureBeat: Observational Memory Cuts AI Agent Costs 10x](https://venturebeat.com/data/observational-memory-cuts-ai-agent-costs-10x-and-outscores-rag-on-long)

#### 2.3.3 Obsidian AI 통합 트렌드

Obsidian은 2026년 AI 통합을 통해 passive에서 active 지식 관리로 진화했습니다:

- **AI Plugins**: 수십 개의 AI 플러그인으로 생성, 요약, 검색, 대화 기능 지원
- **Semantic Search**: 의미 기반 검색으로 정확한 노트 찾기 가능
- **Active Knowledge Management**: 정적 저장소에서 능동적 작업 파트너로 전환
- **Local Processing**: 모든 작업이 로컬에서 처리되어 프라이버시 보장

**출처**:
- [Elephas: Mastering Obsidian in 2026](https://elephas.app/blog/obsidian-guide)
- [GetOpenClaw: Obsidian AI Plugins Complete Guide](https://www.getopenclaw.ai/tools/obsidian-ai)

### 2.4 Strategic Fit

OC-Memory는 세 가지 핵심 기술의 시너지를 극대화합니다:

```
┌─────────────────────────────────────────────────────────────┐
│               Strategic Technology Integration               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  OpenClaw (157K+ stars)                                      │
│  + 폭발적 성장세                                             │
│  + 활발한 커뮤니티                                           │
│  + 범용 AI 에이전트 프레임워크                               │
│                                                              │
│  Mastra Observational Memory (94.87%)                        │
│  + 최고 성능 메모리 시스템                                   │
│  + 검증된 압축 알고리즘                                      │
│  + 산업 표준 벤치마크                                        │
│                                                              │
│  Obsidian + AI (Active KM)                                   │
│  + AI 통합 지식 관리                                         │
│  + 로컬 우선, 프라이버시 보장                                │
│  + 강력한 플러그인 생태계                                    │
│                                                              │
│  = OC-Memory                                                 │
│    차세대 AI 에이전트 메모리 시스템                          │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Stakeholders

### 3.1 Stakeholder Matrix

| Stakeholder | Role | Interest | Influence | Needs |
|-------------|------|----------|-----------|-------|
| **OpenClaw 개발자** | 사용자 | 높음 | 높음 | Zero-Modification, 손쉬운 통합 |
| **OpenClaw 커뮤니티** | 사용자/피드백 | 높음 | 중간 | 문서화, 예제 코드 |
| **AI 연구자** | 평가자 | 중간 | 중간 | 벤치마크 결과, 논문 자료 |
| **기업 사용자** | 고객 | 높음 | 높음 | 보안, 확장성, 지원 |
| **Obsidian 사용자** | 보조 사용자 | 중간 | 낮음 | Obsidian 통합, 클라우드 동기화 |
| **오픈소스 기여자** | 개발자 | 중간 | 중간 | 깔끔한 코드, 기여 가이드 |

### 3.2 Stakeholder Needs Detail

#### 3.2.1 OpenClaw 개발자

**Needs**:
- OpenClaw 코드를 전혀 수정하지 않고 메모리 기능 추가
- 설치 및 설정이 5분 이내 완료
- 명확한 문서화 및 예제 코드
- OpenClaw 업데이트에 영향 받지 않는 안정성
- 초보자도 쉽게 설정할 수 있는 인터랙티브 설치 마법사

**Success Criteria**:
- System Prompt 수정만으로 작동
- 별도 프로세스로 실행되어 OpenClaw와 독립적
- 재시작 시 자동으로 복구
- TUI 설치 마법사로 5분 이내 설정 완료
- API 키 유효성 자동 검증

#### 3.2.2 기업 사용자

**Needs**:
- 데이터 프라이버시 보장 (로컬 저장)
- API 키 보안 관리
- 확장 가능한 아키텍처
- 모니터링 및 알림 기능

**Success Criteria**:
- 모든 데이터 로컬 저장 (클라우드 전송 옵션)
- 파일 권한 600, API 키 환경변수 관리
- 1년 이상 안정적 운영 가능
- 로그 및 메트릭 제공

#### 3.2.3 Obsidian 사용자

**Needs**:
- AI 에이전트의 메모리를 Obsidian에서 조회 및 관리
- Obsidian Vault에 자동 아카이브
- 그래프 뷰로 메모리 연결 시각화
- iCloud/Dropbox 자동 동기화

**Success Criteria**:
- Obsidian CLI 통합 완료
- 90일 후 자동 아카이브
- 양방향 조회 (OpenClaw ↔ Obsidian)
- 클라우드 백업 자동화

---

## 4. Business Objectives

### 4.1 Primary Objectives

#### Objective 1: 장기 기억 능력 구현

**Description**: OpenClaw가 90일 이상 대화 컨텍스트를 유지하고, 사용자 선호 및 과거 작업을 기억

**Measurable Goals**:
- 메모리 유지 기간: 90일 이상 (Hot Memory)
- 검색 정확도: 85% 이상 (시맨틱 검색)
- 컨텍스트 복원율: 95% 이상

**Business Impact**:
- 사용자 만족도 30% 향상
- 반복 질문 80% 감소
- 작업 효율성 50% 향상

#### Objective 2: 토큰 비용 90% 절감

**Description**: Mastra Observational Memory의 압축 기술을 적용하여 토큰 사용량을 90% 이상 절감

**Measurable Goals**:
- 토큰 절약률: ≥90%
- 압축률: 5-10배 (Mastra 기준: 5-40배)
- 월간 API 비용: $150 → $15

**Business Impact**:
- 연간 비용 절감: $1,620
- ROI: 900% (10배)
- 사용자 당 운영 비용 감소

#### Objective 3: Zero-Core-Modification 원칙 준수

**Description**: OpenClaw 코어 코드를 전혀 수정하지 않고 사이드카 패턴으로 메모리 기능 제공

**Measurable Goals**:
- OpenClaw 코드 변경: 0줄
- System Prompt 수정만으로 작동
- OpenClaw 업데이트 호환성: 100%

**Business Impact**:
- 유지보수 비용 제로
- OpenClaw 업데이트 걱정 없음
- 다른 AI 에이전트에도 적용 가능

### 4.2 Secondary Objectives

#### Objective 4: Obsidian + Dropbox 통합 생태계 구축

**Description**: Obsidian을 장기 메모리 저장소로 활용하고, Dropbox로 클라우드 동기화

**Measurable Goals**:
- Obsidian CLI 통합 완료
- 90일 후 자동 아카이브
- 양방향 조회 (OpenClaw ↔ Obsidian ↔ Dropbox)

**Business Impact**:
- 데이터 백업 자동화
- 다른 기기에서도 메모리 접근 가능
- 지식 그래프로 메모리 시각화

#### Objective 5: 벤치마크 검증

**Description**: LongMemEval 벤치마크로 성능 검증 및 Mastra 수준 달성

**Measurable Goals**:
- LongMemEval 점수: ≥85%
- Mastra 벤치마크 (94.87%)의 90% 수준

**Business Impact**:
- 기술적 신뢰성 확보
- 학계 및 산업계 인정
- 논문 발표 가능

### 4.3 Expected ROI

#### 정량적 ROI

```
초기 투자:
- 개발 비용: $0 (오픈소스, 자체 개발)
- 인프라 비용: $0 (로컬 실행)
- 총 투자: $0

연간 절감 효과:
- API 비용 절감: $1,620/년
- 생산성 향상: $2,000/년 (작업 시간 50% 절감)
- 총 효과: $3,620/년

ROI = ∞ (투자 $0, 리턴 $3,620)
```

#### 정성적 ROI

- **사용자 경험**: 맥락 손실 없는 자연스러운 대화
- **신뢰도**: AI 에이전트가 사용자를 "이해"함
- **브랜드 가치**: OpenClaw 생태계의 핵심 컴포넌트로 자리매김
- **커뮤니티**: 157,000+ OpenClaw 사용자에게 어필

---

## 5. Success Metrics

### 5.1 KPI Definition

#### Technical KPIs

| KPI | 측정 방법 | 목표값 | 현재값 | 달성 기한 |
|-----|----------|--------|--------|-----------|
| **Token Savings** | (원본 - 압축) / 원본 × 100% | ≥90% | 0% | 2026-03-31 |
| **Compression Ratio** | 원본 토큰 / 압축 토큰 | 5-10x | 1x | 2026-03-31 |
| **Search Accuracy** | 정답률 (LongMemEval 기준) | ≥85% | - | 2026-04-30 |
| **Memory Retention** | 메모리 유지 기간 (일) | ≥90 | 0 | 2026-03-31 |
| **Response Latency** | 메모리 로드 시간 (ms) | ≤1000 | - | 2026-03-31 |

#### Business KPIs

| KPI | 측정 방법 | 목표값 | 현재값 | 달성 기한 |
|-----|----------|--------|--------|-----------|
| **Cost Reduction** | 월간 API 비용 감소율 | 90% | 0% | 2026-04-30 |
| **User Satisfaction** | 사용자 설문 (1-5점) | ≥4.5 | - | 2026-06-30 |
| **Setup Completion Rate** | 설치 시작 대비 완료율 | ≥95% | - | 2026-04-30 |
| **Adoption Rate** | OC-Memory 설치 사용자 수 | 5,000 | 0 | 2026-12-31 |
| **Zero Downtime** | 시스템 가용성 (%) | 99.9% | - | 지속 |

### 5.2 Success Criteria

#### Phase 1: MVP (2026-03-31)

✅ **Must Have**:
- [ ] 로그 모니터링 (Tail 기반)
- [ ] Observer 에이전트 (LLM 기반)
- [ ] active_memory.md 생성
- [ ] OpenClaw System Prompt 통합
- [ ] 토큰 절약률 90% 이상

⚠️ **Should Have**:
- [ ] ChromaDB 통합
- [ ] Semantic Search
- [ ] TTL 로직 (90일)

❌ **Nice to Have**:
- [ ] Obsidian 연동
- [ ] LongMemEval 벤치마크

#### Phase 2: Production Ready (2026-04-30)

✅ **Must Have**:
- [ ] ChromaDB 통합 완료
- [ ] Semantic Search 85% 정확도
- [ ] TTL 자동 아카이브
- [ ] 문서화 완료

⚠️ **Should Have**:
- [ ] Obsidian CLI 통합
- [ ] Dropbox 동기화
- [ ] LongMemEval ≥85%

#### Phase 3: Ecosystem (2026-06-30)

✅ **Must Have**:
- [ ] Obsidian 양방향 조회
- [ ] Dropbox API 통합
- [ ] 사용자 5,000명 달성

### 5.3 Measurement Plan

#### 데이터 수집

```yaml
metrics_collection:
  # 기술 메트릭
  technical:
    - token_usage: 실시간 수집 (tiktoken)
    - compression_ratio: 배치 계산 (매일)
    - search_accuracy: LongMemEval 벤치마크 (월간)
    - response_latency: 프로메테우스 (실시간)

  # 비즈니스 메트릭
  business:
    - cost_reduction: API 청구서 분석 (월간)
    - user_satisfaction: 설문 조사 (분기)
    - adoption_rate: GitHub Analytics (주간)
    - uptime: 시스템 로그 (실시간)
```

#### 리포팅

- **주간**: 기술 메트릭 대시보드
- **월간**: 비즈니스 KPI 리포트
- **분기**: 사용자 만족도 조사 및 분석
- **연간**: ROI 계산 및 전략 리뷰

---

## 6. Constraints & Assumptions

### 6.1 Technical Constraints

#### Hard Constraints (절대 제약)

| 제약 | 설명 | 영향 |
|------|------|------|
| **Zero-Core-Modification** | OpenClaw 코어 코드 절대 수정 금지 | 사이드카 패턴 필수 |
| **Local First** | 로컬 실행 우선, 클라우드 의존 최소화 | 로컬 DB 사용 (ChromaDB) |
| **Backward Compatibility** | OpenClaw 업데이트와 호환성 유지 | System Prompt만 수정 |
| **Privacy First** | 사용자 데이터 외부 전송 금지 (옵션) | 클라우드 동기화는 선택 사항 |

#### Soft Constraints (권장 제약)

| 제약 | 설명 | 대안 |
|------|------|------|
| **Python 3.8+** | Python 3.8 이상 필요 | PyInstaller로 바이너리 배포 |
| **500MB Storage** | 최소 저장 공간 | TTL로 관리 |
| **LLM API Access** | Observer/Reflector용 LLM API 필요 | 로컬 LLM 지원 추가 |

### 6.2 Business Constraints

#### Budget Constraints

```
예산: $0 (오픈소스 프로젝트)
- 개발 비용: $0 (자체 개발)
- 인프라 비용: $0 (로컬 실행)
- 마케팅 비용: $0 (GitHub, 커뮤니티)

→ 제약: 유료 서비스 사용 불가 (AWS, GCP 등)
→ 대안: 오픈소스 도구만 사용 (ChromaDB, Obsidian CLI)
```

#### Resource Constraints

```
팀: 1명 (Argo)
시간: 6주 (2026-02-12 ~ 2026-03-31)

→ 제약: MVP에 집중, Nice-to-Have 후순위
→ 대안: 커뮤니티 기여자 모집
```

#### Legal Constraints

```
라이선스: MIT (OpenClaw와 동일)
- 상업적 사용 가능
- 수정 및 재배포 가능
- 보증 없음 (AS-IS)

→ 제약: 특허 침해 방지
→ 대안: 오픈소스 라이브러리만 사용
```

### 6.3 Assumptions

#### Technical Assumptions

✅ **Valid Assumptions**:
- OpenClaw는 로그 파일을 생성한다 (`~/.openclaw/logs/*.log`)
- OpenClaw는 System Prompt를 읽는다 (AGENTS.md, SOUL.md 등)
- LLM API는 안정적으로 작동한다 (Google, OpenAI 등)
- ChromaDB는 로컬에서 안정적으로 작동한다

⚠️ **Risky Assumptions**:
- OpenClaw 로그 포맷이 자주 변경되지 않는다
  - **대응**: 로그 파서를 유연하게 설계
- Obsidian CLI가 안정적으로 작동한다
  - **대응**: 직접 파일 조작 대안 제공

#### Business Assumptions

✅ **Valid Assumptions**:
- OpenClaw 사용자는 메모리 기능을 원한다
- 토큰 비용 절감은 중요한 가치다
- 오픈소스 커뮤니티는 활발하게 기여할 것이다

⚠️ **Risky Assumptions**:
- OpenClaw 성장세가 지속된다
  - **대응**: 다른 AI 에이전트에도 적용 가능하도록 설계
- Obsidian 사용자가 AI 에이전트에 관심이 있다
  - **대응**: Obsidian 연동을 선택 사항으로 유지

### 6.4 LLM 모델 선택 전략 (LLM Model Selection Strategy)

#### 비즈니스 관점의 모델 선택

OC-Memory의 Observer와 Reflector는 **중급 수준의 LLM**으로 충분히 작동합니다. 고가의 최상위 모델(GPT-4o, Claude Opus)은 불필요하며, 비용 효율적인 모델 선택이 ROI를 극대화합니다.

#### 추천 모델 (우선순위 순)

##### 1순위: Google Gemini 2.5 Flash (강력 권장)

**선택 이유**:
- 💰 **최저 비용**: $0.075/1M 토큰 (GPT-4o 대비 66배 저렴)
- ✅ **검증된 성능**: Mastra 벤치마크 94.87% LongMemEval 달성
- 🎁 **무료 티어**: 15 RPM (소규모 사용자 무료)
- 🚀 **빠른 속도**: Flash 모델 특성상 응답 속도 우수

**비즈니스 임팩트**:
- 연간 API 비용: ~$0.84 (OpenClaw 절감액의 0.1%)
- ROI: 무한대에 가까움 (비용 $1 → 절감 $809)

##### 2순위: OpenAI GPT-4o-mini

**선택 이유**:
- 🔑 **키 재사용**: 기존 OpenAI 키 활용 가능
- ✅ **우수한 품질**: GPT-4 계열 성능
- ⚡ **빠른 도입**: 별도 계정 불필요

**비즈니스 임팩트**:
- 연간 API 비용: ~$2.40 (Gemini 대비 3배)
- ROI: 여전히 매우 높음 (비용 $2.4 → 절감 $807)

##### 3순위: Claude 3 Haiku

**선택 이유**:
- 📊 **균형잡힌 성능**: 품질과 비용의 균형
- 🔒 **프라이버시**: Anthropic의 데이터 정책

**비즈니스 임팩트**:
- 연간 API 비용: ~$3.00
- ROI: 높음 (비용 $3 → 절감 $806)

#### ❌ 비추천: 고급 모델

**GPT-4o, Claude Opus, Gemini Pro** 등은:
- ❌ **과잉 성능**: Observer/Reflector는 요약 작업만 수행 (고급 추론 불필요)
- ❌ **비용 낭비**: 10-50배 비용 증가로 ROI 하락
- ❌ **속도 저하**: 대형 모델 특성상 응답 느림

#### 필요한 AI 수준

| 능력 | 요구 수준 | 이유 |
|------|----------|------|
| 텍스트 요약 | ⭐⭐⭐ 중급 | Observer/Reflector 핵심 기능 |
| 구조화 | ⭐⭐⭐ 중급 | Observation 포맷 생성 |
| 패턴 인식 | ⭐⭐ 기본 | Reflection 압축 |
| 고급 추론 | ⭐ 불필요 | 단순 요약이므로 |
| 코드 생성 | ⭐ 불필요 | 코드 작성 안 함 |

**결론**: 중급 모델(Flash, mini, Haiku)로 충분

#### 비용 시뮬레이션 (월 사용량 기준)

| 모델 | 토큰 비용 | 월간 호출 | 월 비용 | 연 비용 | ROI |
|------|----------|----------|---------|---------|-----|
| Gemini 2.5 Flash | $0.075/1M | 875회 | $0.07 | $0.84 | ⭐⭐⭐⭐⭐ |
| GPT-4o-mini | $0.15/1M | 875회 | $0.13 | $1.56 | ⭐⭐⭐⭐ |
| Claude 3 Haiku | $0.25/1M | 875회 | $0.22 | $2.64 | ⭐⭐⭐⭐ |
| GPT-4o | $5.00/1M | 875회 | $4.38 | $52.56 | ⭐⭐ |

#### 권장 사항

**스타트업/개인**: Gemini 2.5 Flash (무료 티어 활용)
**기업 (OpenAI 계약)**: GPT-4o-mini (기존 키 재사용)
**보수적 선택**: Claude 3 Haiku (안정적)

### 6.5 Dependencies

#### External Dependencies

| 의존성 | 버전 | 필수 여부 | 대안 |
|--------|------|-----------|------|
| **Python** | 3.8+ | 필수 | PyInstaller 바이너리 |
| **OpenClaw** | Latest | 필수 | 다른 AI 에이전트 지원 추가 |
| **ChromaDB** | 0.4+ | 권장 | 파일 기반 대안 |
| **Obsidian CLI** | Latest | 선택 | 직접 파일 조작 |
| **LLM API** | - | 필수 | 로컬 LLM 지원 |

#### Internal Dependencies

- **tiktoken**: 토큰 계산
- **PyYAML**: 설정 파일 파싱
- **watchdog**: 파일 모니터링 (선택)

---

## 7. Project Scope

### 7.1 In Scope

#### Phase 1: MVP (6주, 2026-02-12 ~ 2026-03-31)

✅ **Core Features**:
- [ ] LogWatcher: 로그 파일 실시간 감시 (Tail 기반)
- [ ] Observer: 로그 분석 및 Observation 생성 (LLM)
- [ ] MemoryMerger: active_memory.md 파일 생성/업데이트
- [ ] TokenCounter: 토큰 계산 (tiktoken)
- [ ] OpenClaw 통합: System Prompt 수정으로 메모리 파일 읽기

✅ **User Experience**:
- [ ] Interactive Setup Wizard (TUI): questionary 기반 6단계 인터랙티브 설정
  - Obsidian/Dropbox 선택적 활성화
  - API 키 자동 저장 (.env)
  - 유효성 검증 자동화
  - 초보자 친화적 UI

✅ **Configuration**:
- [ ] config.yaml: 설정 파일
- [ ] .env: API 키 관리
- [ ] state file: 상태 추적

✅ **Documentation**:
- [ ] README.md: 설치 및 사용법
- [ ] INSTALL.md: 설치 가이드 (TUI 마법사 사용법 포함)
- [ ] API.md: API 문서

#### Phase 2: Production Ready (4주, 2026-04-01 ~ 2026-04-30)

✅ **Advanced Features**:
- [ ] ChromaDB 통합: 벡터 저장소
- [ ] Semantic Search: 의미 기반 검색
- [ ] TTL Manager: 자동 아카이브 (Hot → Warm)
- [ ] Reflector: Observation 압축 (선택)

✅ **Testing**:
- [ ] Unit Tests: pytest
- [ ] Integration Tests: End-to-End
- [ ] Benchmark: LongMemEval

#### Phase 3: Ecosystem (8주, 2026-05-01 ~ 2026-06-30)

✅ **Obsidian Integration**:
- [ ] Obsidian CLI 통합 (Yakitrak)
- [ ] Dropbox API 연동
- [ ] 양방향 조회 (OpenClaw ↔ Obsidian ↔ Dropbox)

✅ **Monitoring & Operations**:
- [ ] 로그 시스템
- [ ] 메트릭 수집 (Prometheus 호환)
- [ ] 알림 시스템

### 7.2 Out of Scope

#### Not in Current Release

❌ **Explicitly Excluded**:
- OpenClaw 코어 코드 수정
- 다른 AI 에이전트 프레임워크 지원 (LangGraph, CrewAI 등)
- 웹 UI / 대시보드
- 클라우드 서비스 (AWS, GCP, Azure)
- 멀티 사용자 지원
- 엔터프라이즈 기능 (SSO, RBAC 등)

#### Future Consideration

🔮 **Potential Future Features**:
- 다른 AI 에이전트 지원 (LangGraph, CrewAI, AutoGPT 등)
- 웹 UI / 대시보드
- 지식 그래프 시각화 (Zep Graphiti 방식)
- 멀티 모달 메모리 (이미지, 오디오 등)
- 협업 메모리 (팀 공유)

### 7.3 Success Definition

#### MVP Success (Phase 1)

```
MVP는 다음 조건을 만족하면 성공:
✅ OpenClaw 코드 수정 없이 작동
✅ 토큰 절약률 90% 이상
✅ 90일 메모리 유지
✅ TUI 설치 마법사로 5분 이내 설정 완료
✅ 설치 완료율 95% 이상 (초보자 친화적)
✅ API 키 유효성 자동 검증
✅ 문서화 완료
```

#### Production Success (Phase 2)

```
Production Ready는 다음 조건을 만족하면 성공:
✅ ChromaDB 통합 완료
✅ Semantic Search 정확도 85% 이상
✅ LongMemEval ≥85%
✅ 자동 아카이브 (TTL) 작동
✅ 50명 이상 사용자 피드백
```

#### Ecosystem Success (Phase 3)

```
Ecosystem은 다음 조건을 만족하면 성공:
✅ Obsidian + Dropbox 통합 완료
✅ 양방향 조회 작동
✅ 5,000명 이상 사용자
✅ 커뮤니티 기여자 10명 이상
✅ GitHub 1,000 stars 이상
```

---

## 8. Risk Assessment

### 8.1 Major Risks

#### Risk Matrix

| Risk ID | Risk | Probability | Impact | Severity |
|---------|------|-------------|--------|----------|
| R1 | OpenClaw 로그 포맷 변경 | 중간 | 높음 | 🔴 High |
| R2 | LLM API 장애 | 중간 | 높음 | 🔴 High |
| R3 | ChromaDB 성능 이슈 | 낮음 | 중간 | 🟡 Medium |
| R4 | Obsidian CLI 호환성 | 중간 | 낮음 | 🟡 Medium |
| R5 | 토큰 비용 초과 | 낮음 | 중간 | 🟡 Medium |
| R6 | 커뮤니티 기여 부족 | 중간 | 낮음 | 🟢 Low |

### 8.2 Risk Details & Mitigation

#### R1: OpenClaw 로그 포맷 변경 🔴

**Risk Description**:
OpenClaw가 업데이트되면서 로그 파일 포맷이 변경되어 OC-Memory가 작동하지 않을 수 있습니다.

**Probability**: 중간 (30%)
- OpenClaw는 활발하게 개발 중
- 로그 포맷은 비교적 안정적이지만 변경 가능성 존재

**Impact**: 높음
- OC-Memory 전체 시스템 중단
- 사용자 불만 증가
- 긴급 패치 필요

**Mitigation Strategy**:
```python
# 1. 유연한 로그 파서 설계
class FlexibleLogParser:
    def __init__(self):
        self.parsers = [
            JSONLogParser(),
            PlainTextLogParser(),
            StructuredLogParser()
        ]

    def parse(self, line: str):
        for parser in self.parsers:
            try:
                return parser.parse(line)
            except:
                continue
        return None  # 파싱 실패

# 2. OpenClaw 버전 감지
class VersionDetector:
    def detect_openclaw_version(self) -> str:
        # OpenClaw 버전을 감지하여 적절한 파서 선택
        pass

# 3. 폴백 메커니즘
# 파싱 실패 시 raw 텍스트로 저장
```

**Contingency Plan**:
- OpenClaw GitHub 모니터링 (releases, commits)
- 사용자 피드백 즉시 대응 (GitHub Issues)
- 긴급 패치 48시간 이내 배포

#### R2: LLM API 장애 🔴

**Risk Description**:
Observer/Reflector가 사용하는 LLM API (Google, OpenAI 등)가 장애가 나면 메모리 생성이 중단됩니다.

**Probability**: 중간 (20%)
- LLM API는 일반적으로 안정적
- 하지만 가끔 장애 발생 (rate limit, outage)

**Impact**: 높음
- 메모리 생성 중단
- 컨텍스트 손실 가능성
- 사용자 경험 저하

**Mitigation Strategy**:
```python
# 1. 다중 LLM 지원
class MultiLLMObserver:
    def __init__(self):
        self.providers = [
            GoogleProvider(),
            OpenAIProvider(),
            AnthropicProvider()
        ]

    def observe(self, messages):
        for provider in self.providers:
            try:
                return provider.observe(messages)
            except Exception as e:
                logging.warning(f"{provider} failed: {e}")
                continue
        raise Exception("All LLM providers failed")

# 2. Retry 로직
@retry(max_attempts=3, backoff=2)
def call_llm(messages):
    return llm.chat(messages)

# 3. 로컬 LLM 폴백
# API 실패 시 로컬 LLM (Ollama 등) 사용
```

**Contingency Plan**:
- API 실패 시 로그만 저장 (나중에 재처리)
- 로컬 LLM 추가 지원 (Ollama, LM Studio)
- 사용자에게 API 키 변경 안내

#### R3: ChromaDB 성능 이슈 🟡

**Risk Description**:
ChromaDB가 커지면서 검색 속도가 느려지거나 메모리 사용량이 증가할 수 있습니다.

**Probability**: 낮음 (15%)
- ChromaDB는 일반적으로 안정적
- 하지만 대용량 데이터 (100k+ observations)에서 성능 저하 가능

**Impact**: 중간
- 검색 속도 저하 (500ms → 2-3초)
- 메모리 사용량 증가 (50MB → 200MB)
- 사용자 경험 저하

**Mitigation Strategy**:
```python
# 1. TTL로 DB 크기 제한
class TTLManager:
    def auto_cleanup(self):
        # 90일 이상 된 관찰 자동 삭제
        old_ids = self.memory_store.get_older_than(90)
        self.memory_store.delete(old_ids)

# 2. 인덱싱 최적화
collection = client.get_or_create_collection(
    name="observations",
    metadata={
        "hnsw:space": "cosine",
        "hnsw:construction_ef": 200,  # 인덱싱 품질 향상
        "hnsw:M": 16  # 연결 수 증가
    }
)

# 3. 배치 처리
def batch_search(queries, batch_size=10):
    results = []
    for i in range(0, len(queries), batch_size):
        batch = queries[i:i+batch_size]
        results.extend(collection.query(batch))
    return results
```

**Contingency Plan**:
- TTL 기간 단축 (90일 → 60일)
- ChromaDB → SQLite 대안 제공
- 수동 DB 정리 도구 제공

#### R4: Obsidian CLI 호환성 🟡

**Risk Description**:
Obsidian CLI (Yakitrak)가 업데이트되거나 사용자 환경에서 작동하지 않을 수 있습니다.

**Probability**: 중간 (25%)
- Obsidian CLI는 서드파티 도구
- 모든 OS에서 안정적이지 않을 수 있음

**Impact**: 낮음
- Obsidian 연동 실패
- Cold Memory 백업 불가
- 하지만 Hot/Warm Memory는 정상 작동

**Mitigation Strategy**:
```python
# 1. 직접 파일 조작 대안
class DirectFileWriter:
    def archive_to_obsidian(self, content, path):
        # Obsidian CLI 대신 직접 파일 쓰기
        obsidian_vault = Path("~/Documents/Obsidian/Main")
        file_path = obsidian_vault / path
        file_path.write_text(content)

# 2. Obsidian 연동 옵션화
config:
  obsidian:
    enabled: false  # 기본값 false
    cli: obsidian-cli
    fallback: direct_file  # 폴백 방식

# 3. 사용자 선택
# Obsidian CLI vs 직접 파일 vs Dropbox API
```

**Contingency Plan**:
- Obsidian CLI 실패 시 직접 파일 조작
- Dropbox API로 대체 (클라우드 동기화)
- Obsidian 연동 없이도 작동 (선택 기능)

#### R5: 토큰 비용 초과 🟡

**Risk Description**:
Observer/Reflector의 LLM 호출이 예상보다 많아져서 토큰 비용이 초과될 수 있습니다.

**Probability**: 낮음 (10%)
- 토큰 임계값 설정으로 제어 가능
- 하지만 사용자 대화량이 매우 많으면 비용 증가

**Impact**: 중간
- 월간 API 비용 증가 ($15 → $50)
- 사용자 불만 (예상 비용 초과)

**Mitigation Strategy**:
```python
# 1. 토큰 임계값 설정
config:
  observation:
    message_tokens: 30000  # Observation 트리거
    max_tokens_per_day: 100000  # 일일 최대 토큰

  reflection:
    observation_tokens: 40000  # Reflection 트리거
    max_tokens_per_month: 1000000  # 월간 최대 토큰

# 2. 토큰 사용량 모니터링
class TokenMonitor:
    def check_daily_limit(self):
        if self.today_tokens > self.config.max_tokens_per_day:
            logging.warning("Daily token limit exceeded")
            return False
        return True

# 3. 사용자 알림
# 토큰 사용량이 80% 도달 시 알림
```

**Contingency Plan**:
- 토큰 임계값 동적 조정
- 로컬 LLM으로 전환 (Ollama, LM Studio)
- 사용자에게 비용 추정 도구 제공

#### R6: 커뮤니티 기여 부족 🟢

**Risk Description**:
오픈소스 프로젝트이지만 커뮤니티 기여자가 부족하면 유지보수가 어려워질 수 있습니다.

**Probability**: 중간 (40%)
- 신규 프로젝트는 초기 기여자 확보가 어려움
- OpenClaw 커뮤니티는 활발하지만 OC-Memory는 신규

**Impact**: 낮음
- 개발 속도 저하
- 버그 수정 지연
- 하지만 핵심 개발자 1명으로 운영 가능

**Mitigation Strategy**:
```markdown
# 1. 기여 가이드 작성
CONTRIBUTING.md:
- 코드 스타일
- 테스트 작성법
- PR 프로세스

# 2. Good First Issue 라벨
GitHub Issues에 초보자용 이슈 표시

# 3. 문서화 강화
- API 문서 자동 생성
- 예제 코드 제공
- 비디오 튜토리얼

# 4. 커뮤니티 활성화
- Discord 채널 개설
- 월간 커뮤니티 미팅
- 기여자 인정 (Contributors 페이지)
```

**Contingency Plan**:
- 핵심 개발자 1명으로 유지보수
- 커뮤니티 매니저 역할 분담
- 스폰서십 유도 (GitHub Sponsors)

### 8.3 Risk Monitoring Plan

#### 주간 리스크 리뷰

```
매주 월요일:
1. 각 리스크의 현재 상태 확인
2. 새로운 리스크 식별
3. Mitigation 진행 상황 점검
4. Contingency Plan 업데이트
```

#### 리스크 대시보드

```python
class RiskDashboard:
    def generate_report(self):
        return {
            "high_risks": [r for r in risks if r.severity == "high"],
            "medium_risks": [r for r in risks if r.severity == "medium"],
            "low_risks": [r for r in risks if r.severity == "low"],
            "mitigated_risks": [r for r in risks if r.status == "mitigated"]
        }
```

---

## 9. Approval

### 9.1 Approval Matrix

| Role | Name | Approval | Date | Signature |
|------|------|----------|------|-----------|
| **Project Manager** | Argo (OpenClaw GM) | ✅ Approved | 2026-02-12 | ✅ |
| **Technical Lead** | Argo (OpenClaw GM) | ✅ Approved | 2026-02-12 | ✅ |
| **Business Sponsor** | OpenClaw Community | 🔄 Pending | - | - |
| **Legal Review** | N/A (Open Source) | ✅ N/A | - | - |

### 9.2 Next Steps

#### Immediate Actions (2026-02-12 ~ 2026-02-19)

- [ ] BRD 승인 획득 (OpenClaw 커뮤니티)
- [ ] GitHub 리포지토리 생성 (argo-ai-memory)
- [ ] 개발 환경 설정
- [ ] PRD 업데이트 (BRD 기반)
- [ ] Tech Spec 업데이트 (BRD 기반)

#### Phase 1 Kickoff (2026-02-20)

- [ ] MVP 개발 시작
- [ ] 주간 진행 상황 리포트
- [ ] 커뮤니티 피드백 수집

---

## 10. Appendix

### 10.1 Glossary

| 용어 | 정의 |
|------|------|
| **Observational Memory** | Mastra가 개발한 AI 메모리 시스템, Observer + Reflector 2단계 압축 |
| **Sidecar Pattern** | 메인 애플리케이션 옆에서 독립적으로 실행되는 보조 프로세스 패턴 |
| **Zero-Core-Modification** | 원본 코드를 전혀 수정하지 않고 기능 추가 |
| **LongMemEval** | AI 에이전트 장기 기억 성능을 측정하는 벤치마크 |
| **Hot/Warm/Cold Memory** | 3-Tier 메모리 시스템 (Hot: 활성, Warm: 아카이브, Cold: 백업) |
| **TTL (Time-To-Live)** | 데이터 수명 주기 관리 정책 |
| **Semantic Search** | 의미 기반 검색 (키워드 매칭이 아닌 벡터 유사도) |

### 10.2 References

#### Key Documents

- [OC-Memory PRD](./OC_Memory_PRD.md)
- [OC-Memory Tech Spec](./OC_Memory_Tech_Spec.md)
- [OC-Memory Research Report](./README.md)
- [OC-Memory Comparative Analysis](./OC_Memory_Comparative_Analysis.md)
- [OC-Memory Improvement Strategy](./OC_Memory_Improvement_Strategy.md)

#### External Sources

**OpenClaw (2026)**:
- [CNBC: From Clawdbot to Moltbot to OpenClaw](https://www.cnbc.com/2026/02/02/openclaw-open-source-ai-agent-rise-controversy-clawdbot-moltbot-moltbook.html)
- [Growth Foundry: OpenClaw Viral Growth Case Study](https://growth.maestro.onl/en/articles/openclaw-viral-growth-case-study)
- [DigitalOcean: What is OpenClaw?](https://www.digitalocean.com/resources/articles/what-is-openclaw)

**Mastra Observational Memory**:
- [Mastra Research: Observational Memory](https://mastra.ai/research/observational-memory)
- [Mastra Blog: Announcing Observational Memory](https://mastra.ai/blog/observational-memory)
- [VentureBeat: Observational Memory Cuts AI Agent Costs 10x](https://venturebeat.com/data/observational-memory-cuts-ai-agent-costs-10x-and-outscores-rag-on-long)

**Obsidian AI Integration**:
- [Elephas: Mastering Obsidian in 2026](https://elephas.app/blog/obsidian-guide)
- [GetOpenClaw: Obsidian AI Plugins Complete Guide](https://www.getopenclaw.ai/tools/obsidian-ai)
- [eesel.ai: Obsidian AI Explained](https://www.eesel.ai/blog/obsidian-ai)

### 10.3 Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-02-12 | Argo | 초안 작성 시작 |
| 1.0 | 2026-02-12 | Argo | 최초 완성 버전 (BRD 전체 작성 완료) |

---

## Document End

**Document Status**: ✅ Complete - Ready for Review
**Next Review Date**: 2026-03-12
**Owner**: Argo (OpenClaw General Manager)

---

*이 문서는 OC-Memory 프로젝트의 비즈니스 요구사항을 정의합니다. 기술 구현 세부사항은 [Tech Spec](./OC_Memory_Tech_Spec.md)을, 제품 요구사항은 [PRD](./OC_Memory_PRD.md)를 참조하세요.*
