# OpenClaw 코드베이스 기술 분석 리포트

**분석 날짜**: 2026-02-12
**분석 대상**: https://github.com/openclaw/openclaw.git
**목적**: OC-Memory 프로젝트의 실제 구현 가능성 검증 및 문서 정확화

---

## Executive Summary

OpenClaw는 예상보다 **훨씬 강력한 확장성**을 제공합니다. HTTP API, Webhook 시스템, Plugin 아키텍처, Memory 자동 인덱싱 등 OC-Memory 프로젝트에 필요한 모든 기능이 **이미 구현되어 있습니다**.

### 주요 발견 사항

| 기능 | 상태 | 구현 방법 |
|------|------|-----------|
| HTTP API | ✅ 완전 지원 | WebSocket Gateway (Port 18789) + OpenAI-compatible endpoint |
| Webhook/Hook | ✅ 완전 지원 | 3가지 Hook 시스템 (External Webhooks, Plugin Hooks, Internal Hooks) |
| System Prompt 주입 | ✅ 완전 지원 | 4가지 방법 (openclaw.json, Plugin Hooks, Context Files, Memory Files) |
| 로그 파일 | ✅ 명확 | `~/.openclaw/logs/`, Session Transcripts (JSONL) |
| 설정 파일 | ✅ 명확 | `~/.openclaw/openclaw.json` (JSON 형식) |
| Plugin 시스템 | ✅ 완전 지원 | Full Plugin SDK with Channel/Hook/HTTP extension |
| Memory 인덱싱 | ✅ 자동화 | SQLite + Vector Embeddings + FTS5 |

**핵심 결론**: Zero-Core-Modification 원칙 하에 **모든 요구사항을 구현 가능**합니다.

---

## 1. HTTP API 분석 (P0)

### 1.1 WebSocket Gateway API

**엔드포인트**: `ws://localhost:18789/` (HTTP upgrade)
**구현 파일**: `src/gateway/server-http.ts`

#### 주요 메서드 (60+ 개)

```typescript
// Message Operations
gateway.send({ text: "Hello", channelId: "telegram" })
gateway.agent({ message: "Run task", agentId: "main" })

// Session Management
gateway.sessions.list({ agentId: "main" })
gateway.sessions.history({ sessionKey: "agent:main:123" })

// Channel Operations
gateway.channels.status()

// Configuration
gateway.config.get()
gateway.config.update({ path: "agents.main.systemPrompt", value: "..." })

// Cron Jobs
gateway.cron.list()
gateway.cron.add({ schedule: "*/5 * * * *", action: "wake", agentId: "main" })
```

**인증**: Bearer token via `gateway.auth.token` config

### 1.2 OpenAI-Compatible Endpoint

**엔드포인트**: `POST http://localhost:18789/v1/chat/completions`
**구현 파일**: `src/gateway/openai-http.ts`

#### 요청 예시

```bash
curl http://localhost:18789/v1/chat/completions \
  -H "Authorization: Bearer <gateway-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-3-5-sonnet-20241022",
    "messages": [{"role": "user", "content": "Hello"}],
    "stream": false,
    "user": "external-session-123"
  }'
```

#### 활성화 방법

```json
{
  "gateway": {
    "http": {
      "endpoints": {
        "chatCompletions": {
          "enabled": true
        }
      }
    }
  }
}
```

**특징**:
- Streaming/Non-streaming 지원
- `user` 필드로 세션 관리
- OpenAI SDK와 호환

### 1.3 Control UI (Web Interface)

**엔드포인트**: `http://localhost:18789/`
**기본 활성화**: Yes (`gateway.controlUi.enabled = true`)

---

## 2. Hook/Notification 메커니즘 (P0)

OpenClaw는 **3가지 독립적인 Hook 시스템**을 제공합니다.

### 2.1 External Webhook Hooks

**구현 파일**: `src/gateway/hooks.ts`, `src/gateway/server-http.ts:134-289`
**문서**: `docs/automation/hooks.md`

#### 설정

```json
{
  "hooks": {
    "enabled": true,
    "token": "your-secret-webhook-token",
    "path": "/hooks",
    "maxBodyBytes": 262144,
    "allowedAgentIds": ["*"],
    "mappings": []
  }
}
```

#### 2.1.1 Wake Endpoint

**엔드포인트**: `POST http://localhost:18789/hooks/wake`

```bash
curl -X POST http://localhost:18789/hooks/wake \
  -H "Authorization: Bearer <webhook-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "New memory entry detected: user_preferences.md",
    "mode": "now"
  }'
```

**mode** 옵션:
- `now`: 즉시 에이전트 wake
- `next-heartbeat`: 다음 heartbeat에 전송

#### 2.1.2 Agent Endpoint (가장 강력)

**엔드포인트**: `POST http://localhost:18789/hooks/agent`

```bash
curl -X POST http://localhost:18789/hooks/agent \
  -H "Authorization: Bearer <webhook-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Analyze new memory file: projects/AI/notes.md",
    "name": "OC-Memory-Watcher",
    "agentId": "main",
    "wakeMode": "now",
    "sessionKey": "external:memory-sync:2026-02-12",
    "deliver": true,
    "channel": "telegram",
    "model": "claude-3-5-sonnet-20241022"
  }'
```

**파라미터**:
- `message`: 에이전트에게 전송할 메시지
- `name`: 발신자 이름 (로그용)
- `agentId`: 대상 에이전트 ID
- `sessionKey`: 세션 식별자 (optional)
- `channel`: 응답을 전송할 채널 (optional)

#### 2.1.3 Custom Mappings (GitHub/GitLab Webhooks)

```json
{
  "hooks": {
    "mappings": [
      {
        "path": "/github-push",
        "transform": "$.commits[*].message",
        "targetAction": "wake",
        "agentId": "main"
      }
    ]
  }
}
```

### 2.2 Plugin Hook System

**구현 파일**: `src/plugins/hooks.ts:1-200`

#### 사용 가능한 Hook Points

```typescript
// System Prompt 주입
before_agent_start: async (event, ctx) => {
  return {
    systemPrompt: "Custom prompt from OC-Memory",
    prependContext: "# Memory Context\n\nRecent memories..."
  };
}

// 메시지 가로채기
message_received: async (msg, ctx) => {
  // 메모리 검색 수행
  const memories = await searchMemories(msg.text);
  msg.text = `[Memory Context: ${memories}]\n\n${msg.text}`;
  return msg;
}

// Tool 호출 전처리
before_tool_call: async (toolCall, ctx) => {
  if (toolCall.tool === "read_file") {
    console.log(`[OC-Memory] Reading: ${toolCall.args.path}`);
  }
}

// Tool 결과 후처리
after_tool_call: async (result, ctx) => {
  if (result.tool === "write_file") {
    // 새 파일 작성 시 Memory 인덱싱 트리거
    await indexNewMemory(result.args.path);
  }
}
```

**Plugin 위치**: `~/.openclaw/plugins/oc-memory-hook/index.js`

### 2.3 Internal Hook System

**구현 파일**: `src/hooks/`

#### Bundled Hooks

1. **session-memory** (`enabled: true` 기본값)
   - 세션 스냅샷 자동 저장
   - `~/.openclaw/agents/<agentId>/sessions/<sessionId>.jsonl`

2. **command-logger** (`enabled: false` 기본값)
   - 모든 `/new`, `/reset`, `/stop` 커맨드 기록
   - `~/.openclaw/logs/commands.log` (JSONL 형식)

3. **boot-md**
   - `BOOT.md` 파일 자동 실행 (시작 시)

4. **soul-evil**
   - 동적 System Prompt 주입

#### 활성화 방법

```json
{
  "hooks": {
    "internal": {
      "enabled": true,
      "entries": {
        "command-logger": { "enabled": true },
        "session-memory": { "enabled": true }
      }
    }
  }
}
```

### 2.4 Telegram Bot Integration

**구현 파일**: `src/telegram/webhook.ts`, `src/telegram/bot.ts`

```json
{
  "channels": {
    "telegram": {
      "enabled": true,
      "token": "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
      "allowFrom": ["dm", "group"],
      "webhook": {
        "enabled": true,
        "path": "/telegram-webhook",
        "secretToken": "your-secret-token"
      }
    }
  }
}
```

**특징**:
- Polling mode (기본값)
- Webhook mode (프로덕션 권장)
- Group chat 지원
- File upload/download 지원

---

## 3. System Prompt 주입 (P0)

**구현 파일**: `src/agents/system-prompt.ts`

### 3.1 openclaw.json 설정 방식

```json
{
  "agents": {
    "main": {
      "systemPrompt": "You are an AI assistant with access to a comprehensive memory system managed by OC-Memory.\n\nWhen responding:\n1. Search your memory files for relevant context\n2. Reference specific memories when available\n3. Update memories when learning new information",

      "contextFiles": [
        "CONTEXT.md",
        "~/oc-memory/system-context.md"
      ],

      "workspace": {
        "dir": "~/.openclaw/workspace",
        "bootstrapFiles": [
          "BOOT.md",
          "~/oc-memory/initialization.md"
        ]
      }
    }
  }
}
```

**특징**:
- `systemPrompt`: 직접 텍스트 삽입
- `contextFiles`: 파일 내용 자동 로드 및 주입
- `bootstrapFiles`: 세션 시작 시 자동 실행

### 3.2 Plugin Hook 방식

**파일**: `~/.openclaw/plugins/oc-memory/index.js`

```javascript
module.exports = {
  name: "oc-memory-injector",
  hooks: [
    {
      hookName: "before_agent_start",
      handler: async (event, ctx) => {
        const memoryContext = await loadMemoryContext();

        return {
          systemPrompt: `You have access to OC-Memory system.\n\nRecent memories:\n${memoryContext}`,
          prependContext: "# Active Memory Context\n\n..."
        };
      }
    }
  ]
};
```

**장점**:
- 동적 Prompt 생성 가능
- 외부 API 호출 가능 (Memory DB 조회 등)
- 조건부 로직 구현 가능

### 3.3 Memory File 자동 주입

**디렉토리**: `~/.openclaw/workspace/memory/*.md`

```markdown
<!-- ~/.openclaw/workspace/memory/MEMORY.md -->

# System Memory

## User Preferences
- Prefers concise responses
- Works on AI/ML projects
- Uses Python and TypeScript

## Project Context
- Current project: OC-Memory integration
- Goal: Zero-modification OpenClaw extension
```

**특징**:
- 자동 인덱싱 (Vector + FTS5)
- `memory_search` tool로 에이전트가 검색 가능
- 파일 변경 시 자동 재인덱싱 (chokidar watching)

### 3.4 Context Files 방식

**우선순위 순서**:
1. System Prompt (`agents.<id>.systemPrompt`)
2. Context Files (`agents.<id>.contextFiles`)
3. Bootstrap Files (`agents.<id>.workspace.bootstrapFiles`)
4. Memory Search Results (runtime)

---

## 4. 로그 파일 시스템 (P1)

### 4.1 로그 파일 위치

**State Directory**: `~/.openclaw/` (기본값)

#### 주요 로그 파일

```
~/.openclaw/
├── logs/
│   ├── commands.log          # Command audit log (JSONL)
│   └── gateway.log           # Gateway logs (optional)
├── agents/
│   └── main/
│       ├── sessions/
│       │   ├── <sessionId>.jsonl   # Session transcripts
│       │   └── <sessionId>.jsonl
│       └── memory.db         # Memory vector DB
```

### 4.2 Command Log 포맷

**파일**: `~/.openclaw/logs/commands.log` (JSONL)

```jsonl
{"timestamp":"2026-02-12T14:30:00.000Z","action":"new","sessionKey":"agent:main:main","senderId":"+1234567890","source":"telegram"}
{"timestamp":"2026-02-12T14:31:00.000Z","action":"reset","sessionKey":"agent:main:main","senderId":"user@example.com","source":"discord"}
{"timestamp":"2026-02-12T14:32:00.000Z","action":"stop","sessionKey":"agent:main:main","senderId":"user123","source":"slack"}
```

**활성화**: `hooks.internal.entries.command-logger.enabled = true`

### 4.3 Session Transcript 포맷

**파일**: `~/.openclaw/agents/main/sessions/<sessionId>.jsonl`

```jsonl
{"type":"user","role":"user","content":"Analyze this code","timestamp":"2026-02-12T14:30:00.000Z"}
{"type":"assistant","role":"assistant","content":"Let me read the file...","timestamp":"2026-02-12T14:30:01.000Z"}
{"type":"tool_use","tool":"read_file","args":{"path":"/path/to/file.py"},"timestamp":"2026-02-12T14:30:02.000Z"}
{"type":"tool_result","tool":"read_file","content":"def main():\n    pass","timestamp":"2026-02-12T14:30:03.000Z"}
{"type":"assistant","role":"assistant","content":"The code defines...","timestamp":"2026-02-12T14:30:04.000Z"}
```

**특징**:
- 완전한 대화 히스토리
- Tool 호출 및 결과 포함
- Timestamp 포함
- JSONL 형식 (라인별 JSON)

### 4.4 Subsystem Logs (Console)

**구현 파일**: `src/logging/subsystem.ts`

```
[2026-02-12 14:30:00] [telegram] message received: chatId=123456
[2026-02-12 14:30:01] [gateway/hooks] webhook received: path=/hooks/agent
[2026-02-12 14:30:02] [discord] connected to gateway
[2026-02-12 14:30:03] [agent:main] session started: sessionKey=agent:main:main
```

**포맷 설정**:
```json
{
  "logging": {
    "style": "pretty",  // "pretty" | "compact" | "json"
    "level": "info"     // "debug" | "info" | "warn" | "error"
  }
}
```

### 4.5 로그 감시 전략

**OC-Memory에서 사용 가능한 방법**:

1. **JSONL Tail Watching**
   ```bash
   tail -f ~/.openclaw/logs/commands.log | while read line; do
     echo "$line" | jq -r '.action, .sessionKey'
   done
   ```

2. **Session Transcript Watching**
   ```bash
   # 새 tool_use 이벤트 감지
   tail -f ~/.openclaw/agents/main/sessions/*.jsonl | grep tool_use
   ```

3. **Custom Hook으로 별도 로그 작성**
   ```javascript
   // Plugin hook
   after_tool_call: async (result, ctx) => {
     fs.appendFileSync('/var/log/oc-memory.log',
       JSON.stringify({tool: result.tool, timestamp: Date.now()}) + '\n'
     );
   }
   ```

---

## 5. 설정 파일 구조 (P1)

### 5.1 openclaw.json 전체 구조

**위치**: `~/.openclaw/openclaw.json`
**Override**: `OPENCLAW_CONFIG_PATH` 환경변수

```json
{
  "gateway": {
    "mode": "local",
    "port": 18789,
    "bind": "loopback",
    "auth": {
      "token": "generated-on-first-run",
      "password": null,
      "allowTailscale": true
    },
    "trustedProxies": [],
    "controlUi": {
      "enabled": true,
      "basePath": "/"
    },
    "http": {
      "endpoints": {
        "chatCompletions": {
          "enabled": false
        },
        "responses": {
          "enabled": false
        }
      }
    }
  },

  "agents": {
    "main": {
      "systemPrompt": null,
      "contextFiles": [],
      "workspace": {
        "dir": "~/.openclaw/workspace",
        "bootstrapFiles": []
      },
      "memory": {
        "enabled": true,
        "provider": "openai",
        "model": "text-embedding-3-small",
        "chunkTokens": 512,
        "chunkOverlap": 128,
        "extraPaths": []
      },
      "tools": {
        "policy": "allow-all"
      },
      "defaultModel": "claude-3-5-sonnet-20241022"
    }
  },

  "hooks": {
    "enabled": true,
    "token": "generated-webhook-token",
    "path": "/hooks",
    "maxBodyBytes": 262144,
    "allowedAgentIds": ["*"],
    "mappings": [],
    "internal": {
      "enabled": true,
      "entries": {
        "session-memory": {
          "enabled": true,
          "compress": true
        },
        "command-logger": {
          "enabled": false
        },
        "boot-md": {
          "enabled": true
        }
      }
    }
  },

  "channels": {
    "telegram": {
      "enabled": false,
      "token": null,
      "allowFrom": ["dm", "group"],
      "webhook": {
        "enabled": false,
        "path": "/telegram-webhook",
        "secretToken": null
      }
    },
    "discord": {
      "enabled": false,
      "token": null,
      "applicationId": null
    },
    "slack": {
      "enabled": false,
      "appToken": null,
      "botToken": null
    }
  },

  "providers": {
    "anthropic": {
      "apiKey": null,
      "baseUrl": null
    },
    "openai": {
      "apiKey": null,
      "baseUrl": null
    },
    "google": {
      "apiKey": null
    }
  },

  "logging": {
    "style": "pretty",
    "level": "info"
  }
}
```

### 5.2 환경변수 우선순위

**파일**: `~/.openclaw/.env` 또는 `./.env`

```bash
# Gateway
OPENCLAW_GATEWAY_TOKEN=your-custom-token
OPENCLAW_GATEWAY_PORT=18789

# State Directory
OPENCLAW_STATE_DIR=/custom/path
OPENCLAW_HOME=/custom/home

# API Keys
ANTHROPIC_API_KEY=sk-ant-api03-...
OPENAI_API_KEY=sk-...

# Telegram
TELEGRAM_BOT_TOKEN=123456:ABC-DEF...

# Config Path Override
OPENCLAW_CONFIG_PATH=/custom/config.json
```

**우선순위**:
1. 환경변수 (최우선)
2. `.env` 파일
3. `openclaw.json`
4. 기본값

### 5.3 경로 해석 규칙

**구현 파일**: `src/config/paths.ts`

```typescript
// Tilde expansion
"~/memory" → "/home/user/memory"

// Relative paths (workspace 기준)
"CONTEXT.md" → "~/.openclaw/workspace/CONTEXT.md"

// Absolute paths
"/etc/openclaw/config.md" → "/etc/openclaw/config.md"

// Environment variable expansion
"${HOME}/memory" → "/home/user/memory"
```

---

## 6. Plugin/Extension 시스템 (P2)

### 6.1 Plugin SDK

**구현 파일**: `src/plugin-sdk/index.ts`

#### Plugin 구조

```javascript
// ~/.openclaw/plugins/oc-memory/index.js
module.exports = {
  name: "oc-memory-integration",
  version: "1.0.0",

  // HTTP Routes
  routes: [
    {
      method: "POST",
      path: "/oc-memory/sync",
      handler: async (req, res, ctx) => {
        const { filePath } = req.body;
        await syncMemoryFile(filePath);
        res.json({ success: true });
      }
    }
  ],

  // Hook Registrations
  hooks: [
    {
      hookName: "before_agent_start",
      handler: async (event, ctx) => {
        return {
          systemPrompt: await generateMemoryPrompt()
        };
      }
    },
    {
      hookName: "after_tool_call",
      handler: async (result, ctx) => {
        if (result.tool === "write_file") {
          await indexMemoryFile(result.args.path);
        }
      }
    }
  ],

  // Channel Extension (optional)
  channel: {
    id: "oc-memory-notifications",
    start: async (ctx) => {
      // Start notification listener
    },
    stop: async (ctx) => {
      // Cleanup
    }
  }
};
```

### 6.2 Plugin 설치 위치

```
~/.openclaw/
├── plugins/
│   ├── oc-memory/
│   │   ├── index.js
│   │   ├── package.json
│   │   └── node_modules/
│   └── other-plugin/
└── openclaw.json
```

### 6.3 기존 Extension 예시

OpenClaw에 이미 구현된 Extension들:

1. **MS Teams** (`extensions/msteams/`)
2. **BlueBubbles (iMessage)** (`extensions/bluebubbles/`)
3. **Feishu/Lark** (`extensions/feishu/`)
4. **Voice Call** (`extensions/voice-call/`)
5. **Google Chat** (`extensions/googlechat/`)

---

## 7. Memory 파일 인덱싱 (P2)

### 7.1 자동 인덱싱 메커니즘

**구현 파일**: `src/memory/sync-memory-files.ts`, `src/memory/manager.ts`

#### 인덱싱 프로세스

```
1. File Discovery
   ├─ ~/.openclaw/workspace/MEMORY.md
   ├─ ~/.openclaw/workspace/memory/*.md
   └─ Extra paths (agents.main.memory.extraPaths)

2. File Watching (chokidar)
   ├─ Change detection
   └─ Debounced sync (5초)

3. Text Chunking
   ├─ Chunk size: 512 tokens (configurable)
   ├─ Overlap: 128 tokens
   └─ Markdown-aware splitting

4. Embedding Generation
   ├─ Provider: OpenAI/Google/Voyage/Local
   ├─ Model: text-embedding-3-small
   └─ Caching (embedding_cache table)

5. Database Storage
   ├─ SQLite + sqlite-vec
   ├─ Vector index (chunks_vec)
   └─ FTS5 index (chunks_fts)
```

### 7.2 Memory Database 스키마

**위치**: `~/.openclaw/agents/main/memory.db` (SQLite)

```sql
-- File metadata
CREATE TABLE files (
  id INTEGER PRIMARY KEY,
  path TEXT UNIQUE,
  hash TEXT,
  indexed_at INTEGER,
  metadata TEXT
);

-- Text chunks
CREATE TABLE chunks (
  id INTEGER PRIMARY KEY,
  file_id INTEGER,
  chunk_index INTEGER,
  start_line INTEGER,
  end_line INTEGER,
  text TEXT,
  token_count INTEGER,
  FOREIGN KEY (file_id) REFERENCES files(id)
);

-- Vector embeddings (sqlite-vec)
CREATE VIRTUAL TABLE chunks_vec USING vec0(
  chunk_id INTEGER PRIMARY KEY,
  embedding FLOAT[1536]
);

-- Full-text search (FTS5)
CREATE VIRTUAL TABLE chunks_fts USING fts5(
  chunk_id UNINDEXED,
  text,
  content='chunks',
  content_rowid='id'
);

-- Embedding cache
CREATE TABLE embedding_cache (
  text_hash TEXT PRIMARY KEY,
  embedding BLOB,
  model TEXT,
  created_at INTEGER
);
```

### 7.3 Memory Search Tools

#### Agent가 사용 가능한 Tools

```typescript
// 1. Vector + Keyword Hybrid Search
await agent.tools.memory_search({
  query: "user preferences for AI projects",
  limit: 5,
  threshold: 0.7  // Similarity threshold
});

// 2. Get Specific Lines
await agent.tools.memory_get({
  path: "MEMORY.md",
  startLine: 10,
  endLine: 20
});
```

#### Search Result Format

```json
{
  "results": [
    {
      "file": "memory/user_preferences.md",
      "lines": "15-20",
      "similarity": 0.89,
      "text": "User prefers concise responses and works on AI/ML projects using Python and TypeScript.",
      "context": "## User Preferences\n\n..."
    }
  ]
}
```

### 7.4 Memory 설정

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
        "extraPaths": [
          "~/Documents/notes",
          "/mnt/knowledge-base"
        ],
        "watch": true,
        "syncInterval": 5000
      }
    }
  }
}
```

---

## 8. OC-Memory 구현 전략

### 8.1 권장 아키텍처

```
┌─────────────────────────────────────────────────────┐
│                  OC-Memory Service                  │
│  (Standalone Go/Python/Node.js Service)             │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. File Watcher (chokidar/watchdog)               │
│     └─ Monitor: ~/Documents, ~/Projects, etc.      │
│                                                     │
│  2. HTTP Client                                     │
│     └─ POST to OpenClaw Webhook API                │
│                                                     │
│  3. Memory Analyzer                                 │
│     └─ Extract keywords, categories, importance    │
│                                                     │
│  4. Config Manager                                  │
│     └─ Update openclaw.json dynamically            │
│                                                     │
└─────────────────────────────────────────────────────┘
                        │
                        ▼
         ┌──────────────────────────┐
         │  OpenClaw Gateway API    │
         │  http://localhost:18789  │
         └──────────────────────────┘
                        │
         ┌──────────────┴──────────────┐
         ▼                             ▼
  [Webhook Hooks]              [Memory System]
  POST /hooks/agent            ~/.openclaw/workspace/memory/
```

### 8.2 연동 방법 선택

| 요구사항 | 권장 방법 | 이유 |
|---------|----------|------|
| 실시간 알림 | Webhook `/hooks/agent` | 즉각적 에이전트 wake 가능 |
| System Prompt 주입 | `openclaw.json` + Plugin Hook | 동적/정적 모두 지원 |
| Memory 인덱싱 | `~/.openclaw/workspace/memory/` 직접 작성 | 자동 벡터 인덱싱 |
| 로그 분석 | Session Transcript watching | 완전한 히스토리 포함 |
| 설정 관리 | `openclaw.json` 동적 수정 | Zero-Core-Modification |

### 8.3 구현 우선순위 재조정

**Phase 1 (MVP)**: Webhook + Memory File Writing
- OC-Memory watches files → Writes to `~/.openclaw/workspace/memory/`
- OpenClaw auto-indexes → Agent searches via `memory_search`
- File change → POST to `/hooks/wake`

**Phase 2**: Plugin Hook Development
- `before_agent_start` hook for dynamic prompt injection
- `after_tool_call` hook for memory update detection

**Phase 3**: Advanced Integration
- Custom UI via Plugin HTTP routes
- Cron-based periodic memory sync
- Multi-channel notification (Telegram + Discord)

---

## 9. 기술적 제약 및 고려사항

### 9.1 확인된 제약

1. **HTTP API 포트**: 기본 18789 (변경 가능하나 single port)
2. **Memory Provider**: Embedding API 키 필요 (OpenAI/Google/Voyage)
3. **Plugin 실행 환경**: Node.js 런타임 (CommonJS or ESM)
4. **Webhook 인증**: Token 기반 (API Key 스타일 아님)
5. **Memory 파일 포맷**: Markdown only (`.md` 확장자 필수)

### 9.2 보안 고려사항

1. **Gateway Token 관리**
   - 외부 노출 금지
   - 환경변수로 관리 권장
   - Tailscale 통합 시 추가 보안

2. **Webhook Token 관리**
   - `/hooks/*` 엔드포인트 전용 토큰
   - Gateway token과 분리 관리
   - HTTPS 사용 권장 (프로덕션)

3. **Memory File 권한**
   - `~/.openclaw/workspace/memory/` 권한 관리
   - Sensitive information 암호화 고려

### 9.3 성능 고려사항

1. **Memory Indexing**
   - 대용량 파일 (>10MB) 시 chunking 시간 증가
   - Embedding API rate limit 고려
   - 캐싱 전략 (embedding_cache 활용)

2. **Webhook Rate Limiting**
   - OpenClaw에 내장된 rate limiting 없음
   - OC-Memory 측에서 구현 필요

3. **Session Transcript Size**
   - 장기 세션 시 JSONL 파일 크기 증가
   - 주기적 cleanup 전략 필요

---

## 10. 결론 및 권고사항

### 10.1 주요 발견 요약

1. **HTTP API**: ✅ 완전 지원 - WebSocket Gateway + OpenAI-compatible endpoint
2. **Webhook System**: ✅ 완전 지원 - 3가지 독립적 Hook 시스템
3. **System Prompt 주입**: ✅ 4가지 방법 지원 (openclaw.json, Plugin Hook, Context Files, Memory)
4. **Memory Indexing**: ✅ 자동화 - SQLite + Vector + FTS5
5. **Plugin Architecture**: ✅ 전체 SDK 제공
6. **Log Files**: ✅ 명확 - JSONL 포맷

### 10.2 문서 수정 필요 사항

#### PRD.md
- ~~가정: HTTP API 없을 수 있음~~ → **확정: HTTP API 존재 (WebSocket Gateway)**
- ~~대안: Log file watching~~ → **주방법: Webhook `/hooks/agent`, 보조: Session transcript**
- FR-003 수정: "Log file parsing" → "Webhook-based real-time notification"

#### Tech_Spec.md
- API 명세 구체화: `/hooks/wake`, `/hooks/agent` endpoint 명시
- System Prompt 주입 방법: `openclaw.json` + Plugin Hook 조합
- Memory 경로: `~/.openclaw/workspace/memory/` (자동 인덱싱)
- Config 파일 구조: `openclaw.json` 전체 스키마 추가

#### Tasks.md
- Task 1.1: ~~HTTP API 조사~~ → **완료: Webhook API 명세 작성**
- Task 2.1: File watcher → **Memory file writer로 단순화**
- Task 2.2: Webhook integration → **구체적 endpoint 및 payload 명시**
- Task 3.1: ~~System Prompt 방법 조사~~ → **Plugin Hook 개발**

### 10.3 최종 권고사항

#### Zero-Core-Modification 원칙 달성 가능
- OpenClaw 코드 수정 불필요
- 모든 연동을 외부 서비스 + Plugin으로 구현 가능

#### 권장 기술 스택
- **OC-Memory Service**: Node.js (OpenClaw와 동일 런타임)
- **File Watching**: chokidar (OpenClaw와 동일 라이브러리)
- **HTTP Client**: axios 또는 fetch
- **Plugin**: CommonJS module in `~/.openclaw/plugins/`

#### 구현 순서
1. Basic Webhook Integration (1-2일)
2. Memory File Writer (1일)
3. Plugin Hook Development (2-3일)
4. Telegram Notification (1일)
5. Advanced Features (cron, UI) (3-5일)

**총 예상 개발 기간**: 8-12일 (1 FTE 기준)

---

## 부록 A: 핵심 파일 경로 참조

### HTTP API
- `src/gateway/server-http.ts` - HTTP server & WebSocket gateway
- `src/gateway/openai-http.ts` - OpenAI-compatible endpoint
- `src/gateway/server-methods.ts` - Gateway method handlers

### Webhook & Hooks
- `src/gateway/hooks.ts` - Webhook configuration & handlers
- `src/hooks/` - Internal hook system
- `src/plugins/hooks.ts` - Plugin hook system
- `docs/automation/hooks.md` - Official webhook documentation

### System Prompt
- `src/agents/system-prompt.ts` - System prompt builder
- `src/agents/bootstrap.ts` - Agent bootstrap process

### Memory
- `src/memory/manager.ts` - Memory indexing manager
- `src/memory/sync-memory-files.ts` - File watcher & sync
- `src/memory/memory-schema.ts` - SQLite schema definitions

### Configuration
- `src/config/config.ts` - Config loading & validation
- `src/config/paths.ts` - Path resolution utilities

### Logging
- `src/logging/subsystem.ts` - Subsystem logger
- `src/logging/diagnostic.ts` - Diagnostic event logger

### Plugins
- `src/plugin-sdk/index.ts` - Plugin SDK
- `src/plugins/` - Plugin system core
- `extensions/` - Example extensions

---

## 부록 B: 코드 스니펫 참조

### B.1 Webhook 전송 예시 (Node.js)

```javascript
const axios = require('axios');

async function notifyOpenClaw(message) {
  const response = await axios.post(
    'http://localhost:18789/hooks/agent',
    {
      message: message,
      name: 'OC-Memory',
      agentId: 'main',
      wakeMode: 'now',
      sessionKey: `external:memory:${Date.now()}`
    },
    {
      headers: {
        'Authorization': `Bearer ${process.env.OPENCLAW_WEBHOOK_TOKEN}`,
        'Content-Type': 'application/json'
      }
    }
  );

  return response.data;
}

// Usage
await notifyOpenClaw('New memory file detected: projects/AI/research.md');
```

### B.2 Memory File 작성 예시

```javascript
const fs = require('fs').promises;
const path = require('path');

async function writeMemoryFile(category, content) {
  const memoryDir = path.join(
    process.env.HOME,
    '.openclaw/workspace/memory'
  );

  const filename = `${category}_${Date.now()}.md`;
  const filepath = path.join(memoryDir, filename);

  const markdown = `# ${category}\n\n${content}\n\n---\n*Updated: ${new Date().toISOString()}*\n`;

  await fs.writeFile(filepath, markdown, 'utf-8');

  // OpenClaw will auto-index this file within 5 seconds
  console.log(`Memory written: ${filepath}`);
}

// Usage
await writeMemoryFile('user_preferences', 'User prefers dark mode UI');
```

### B.3 Plugin Hook 예시

```javascript
// ~/.openclaw/plugins/oc-memory/index.js
module.exports = {
  name: 'oc-memory-integration',
  version: '1.0.0',

  hooks: [
    {
      hookName: 'before_agent_start',
      handler: async (event, ctx) => {
        // Load recent memories
        const memories = await loadRecentMemories(5);

        const memoryContext = memories
          .map(m => `- ${m.category}: ${m.summary}`)
          .join('\n');

        return {
          prependContext: `# Recent Memory Context\n\n${memoryContext}\n\n---\n`
        };
      }
    },

    {
      hookName: 'after_tool_call',
      handler: async (result, ctx) => {
        if (result.tool === 'write_file' && result.args.path.includes('.md')) {
          console.log(`[OC-Memory] New markdown file: ${result.args.path}`);
          // Trigger external indexing if needed
        }
      }
    }
  ]
};
```

---

**문서 버전**: 1.0
**분석 도구**: OpenClaw Explore Agent
**분석 완료일**: 2026-02-12
