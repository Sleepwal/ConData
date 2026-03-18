# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ConData is a PostgreSQL database management desktop application built with:
- **Frontend**: Vue 3 + TypeScript + Pinia + Vue Router
- **Backend**: Rust with Tauri 2.x
- **Database**: PostgreSQL via tokio-postgres and deadpool-postgres

## Development Commands

```bash
# Install dependencies
pnpm install

# Start development server (runs Vite dev server + Tauri app)
pnpm tauri dev

# Build frontend only
pnpm build

# Type check TypeScript
pnpm vue-tsc --noEmit

# Build production Tauri app
pnpm tauri build
```

## Architecture

### Frontend (`src/`)

**State Management (Pinia)**:
- `stores/connection.ts` - Manages database connections, connection statuses, and active connection
- `stores/query.ts` - Manages query history and current query state

**API Layer**:
- `api/index.ts` - Wraps all Tauri invoke calls (connectionApi, queryApi)

**Type Safety**:
- `types/index.ts` - Shared TypeScript interfaces that mirror Rust structs

### Backend (`src-tauri/src/`)

**Command Pattern**:
- `commands/connection.rs` - Tauri commands for connection management (test, save, connect, disconnect)
- `commands/query.rs` - Tauri commands for query execution and metadata retrieval

**Connection Pool Management**:
- `db/pool.rs` - Global DashMap-based connection pool storage using deadpool-postgres
- Pools are keyed by connection_id and stored in a static `DashMap<String, Pool>`

**Services**:
- `services/postgres_service.rs` - Executes queries and retrieves database metadata
- `services/connection_manager.rs` - Persists connection configs to local JSON file

**Security**:
- `models/security.rs` - AES-256-GCM encryption for passwords using blake3-derived master key
- Connection configs stored encrypted at OS-specific config directory

**Error Handling**:
- `error.rs` - `AppError` enum with variants: DatabaseError, ConnectionError, ConfigError, ValidationError, NotFound, InternalError, SecurityError
- All errors implement Serialize/Deserialize for Tauri command results

### Key Data Flows

**Connection Flow**:
1. Frontend calls `connectionApi.connect(connectionId)`
2. Backend command retrieves config from `ConnectionManager`
3. `ConnectionPool::connect()` creates pool and stores in global DashMap
4. Pool validated with `SELECT version()` query

**Query Flow**:
1. Frontend calls `queryApi.executeQuery({connection_id, sql, limit})`
2. `PostgresService::execute_query()` retrieves pool from global storage
3. Query executed with optional row limit (default 1000)
4. Results converted to JSON and returned with execution time

## Security Considerations

- Passwords are encrypted using AES-256-GCM before storage
- Master key is derived from app name using blake3 hash
- Connection configs stored at OS config dir (e.g., `%APPDATA%/condata/connections.json` on Windows)
- SQL queries use parameterized queries where applicable (metadata queries)

## CI/CD

GitHub Actions workflow (`.github/workflows/build.yml`) builds for:
- Windows (MSI, NSIS, Portable)
- macOS Intel & Apple Silicon (DMG, APP)
- Linux (DEB, AppImage)

Trigger: Push to main/master, PRs, or manual dispatch
