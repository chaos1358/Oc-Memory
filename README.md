# OC-Memory: OpenClaw Observational Memory System

**OpenClaw에 장기 기억 능력을 부여하는 외장형 메모리 시스템**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Mastra OM](https://img.shields.io/badge/Mastra%20OM-94.87%25-green.svg)](https://mastra.ai/research/observational-memory)

---

## 🎯 프로젝트 개요

OC-Memory는 [OpenClaw](https://openclaw.ai/)에 [Mastra Observational Memory](https://mastra.ai/docs/memory/observational-memory) 개념을 응용하여 **Zero-Core-Modification** 방식으로 장기 기억 능력을 부여하는 사이드카 시스템입니다.

### 핵심 특징

- **🧠 장기 기억**: 90일 이상 대화 맥락 유지
- **💰 90% 토큰 절약**: 5-40배 압축률로 비용 대폭 절감
- **🔒 Zero-Core-Modification**: OpenClaw 코드 수정 없이 동작
- **☁️ 클라우드 백업**: Obsidian + Dropbox 연동으로 안전한 장기 보관
- **🔍 시맨틱 검색**: ChromaDB 기반 의미 기반 검색

---

## 📊 성능 지표

| 지표 | 목표 | 근거 |
|------|------|------|
| **LongMemEval 점수** | 85%+ | Mastra OM 94.87% 달성 |
| **토큰 절약률** | 90%+ | 5-40배 압축률 |
| **압축률** | 5-10x | Mastra OM 벤치마크 |
| **검색 정확도** | 85%+ | ChromaDB 시맨틱 검색 |

---

## 🏗️ 아키텍처

```
┌─────────────────────────────────────────────────────────────┐
│                    3-Tier Memory System                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Hot Memory (0-90일)                                         │
│  - Storage: ChromaDB + active_memory.md                     │
│  - Access: Real-time, Semantic Search                       │
│                                                              │
│  Warm Memory (90-365일)                                      │
│  - Storage: Markdown Archives                               │
│  - Access: On-demand, Grep Search                          │
│                                                              │
│  Cold Memory (365일+)                                        │
│  - Storage: Obsidian Vault + Dropbox                        │
│  - Access: Cloud Sync, Read-only                           │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📚 문서

프로젝트의 모든 문서는 **[specs/](./specs/)** 폴더에 체계적으로 정리되어 있습니다:

### 📋 비즈니스 문서
- **[BRD.md](./specs/BRD.md)** - Business Requirements Document
  - 프로젝트 비전, 목표, 이해관계자, ROI 분석
  - OpenClaw, Mastra OM, Obsidian 2026년 최신 정보 반영

### 🎨 제품 문서
- **[PRD.md](./specs/PRD.md)** - Product Requirements Document
  - 사용자 페르소나, 기능 요구사항, 사용자 스토리
  - 3-Tier Memory System 상세 설계
  - ChromaDB, Obsidian, Dropbox 통합 방안

### 🔧 기술 문서
- **[Tech_Spec.md](./specs/Tech_Spec.md)** - Technical Specification
  - 시스템 아키텍처, API 설계, 데이터 모델
  - Observer & Reflector 구현 사양
  - 성능 요구사항, 보안, 배포 전략

### ✅ 구현 계획
- **[Tasks.md](./specs/Tasks.md)** - Implementation Tasks
  - 4 Phase, 11주 개발 로드맵
  - 67+ 상세 작업, 185 Story Points
  - Sprint 계획, 리스크 관리, QA 전략

---

## 🚀 빠른 시작

### 전제 조건

- Python 3.8+
- OpenClaw 설치 및 실행 중
- Google API Key (Observer/Reflector용)

### 설치 방법

#### 방법 1: 🎨 TUI 설치 마법사 (추천)

**인터랙티브 6단계 설정 - 초보자 친화적**

```bash
# 저장소 클론
git clone https://github.com/[username]/oc-memory.git
cd oc-memory

# 의존성 설치
pip install -r requirements.txt

# TUI 설치 마법사 실행
python setup.py
```

**TUI 설치 마법사 특징**:
- ✅ 6단계 인터랙티브 설정 (5분 이내 완료)
- ✅ Obsidian/Dropbox 선택적 활성화
- ✅ API 키 자동 검증 및 안전 저장
- ✅ 유효성 검사 자동화
- ✅ 초보자도 쉽게 설치

#### 방법 2: ⚙️ 수동 설정 (고급 사용자)

```bash
# 저장소 클론
git clone https://github.com/[username]/oc-memory.git
cd oc-memory

# 의존성 설치
pip install -r requirements.txt

# 설정 파일 생성
cp config.example.yaml config.yaml
# config.yaml 편집 (API 키 등)

# 데몬 시작
python memory_observer.py
```

### OpenClaw 설정

OpenClaw의 System Prompt에 다음 추가:

```markdown
## 🧠 Memory Instructions

Before responding, read your memory file at ~/.openclaw/active_memory.md
to recall past context, user preferences, and ongoing tasks.
```

---

## 🛠️ 기술 스택

| 카테고리 | 기술 |
|----------|------|
| **언어** | Python 3.8+ |
| **벡터 DB** | ChromaDB (Persistent) |
| **LLM API** | OpenAI, Google Gemini |
| **노트 앱** | Obsidian + Yakitrak CLI |
| **클라우드** | Dropbox API |
| **배포** | systemd, macOS LaunchAgent |

---

## 📈 개발 로드맵

- **Phase 1 (4주)**: MVP - 기본 메모리 시스템 + OpenClaw 연동
- **Phase 2 (3주)**: Enhanced - ChromaDB + Semantic Search + TTL
- **Phase 3 (2주)**: Integration - Obsidian + Dropbox 연동
- **Phase 4 (2주)**: Production - 테스트 + 문서화 + 배포

자세한 일정은 [Tasks.md](./specs/Tasks.md) 참조

---

## 🤝 기여

이 프로젝트는 오픈소스이며, 기여를 환영합니다!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 라이선스

MIT License - 자세한 내용은 [LICENSE](LICENSE) 파일 참조

---

## 🙏 감사의 말

- **[Mastra](https://mastra.ai/)**: Observational Memory 개념 및 아키텍처
- **[OpenClaw](https://openclaw.ai/)**: 오픈소스 AI 에이전트 프레임워크
- **[Obsidian](https://obsidian.md/)**: Second Brain 지식 관리 시스템
- **[ChromaDB](https://www.trychroma.com/)**: AI-native 벡터 데이터베이스

---

## 📞 연락처

- 프로젝트 이슈: [GitHub Issues](https://github.com/[username]/oc-memory/issues)
- 문의: [이메일 주소]

---

**Built with ❤️ for the OpenClaw community**
