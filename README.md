# DirectOffice

DirectOffice is a desktop communication and productivity application designed for office environments.

## Architecture
- **Desktop**: Tauri 2 + React + TypeScript + Rust backend
- **Server**: Rust + Axum + PostgreSQL
- **AI**: Local CPU-only speech recognition (whisper.cpp) & local LLM (llama.cpp)
- **Deployment**: Self-hosted on LAN

## Directory Structure
- `Desktop/`: Tauri 2 client application (Frontend, Backend, Assets, Configuration)
- `Server/`: Self-hosted office server (Axum, PostgreSQL, WebSockets, Storage)
- `AI/`: Local CPU runtimes, models, transcription and summarization engines
- `Installer/`: Packaging definitions for Windows and Linux
- `Scripts/`: Automation scripts for build, development, and deployment
- `Documentation/`: Guides and technical specifications
- `Tests/`: Unit, integration, and performance tests

================================================================================
DIRECTOFFICE - HOW TO RUN GUIDE
================================================================================

This guide covers running the DirectOffice Server Backend and Desktop App.

--------------------------------------------------------------------------------
PREREQUISITES
--------------------------------------------------------------------------------
1. Rust (cargo & rustc installed, 2021 edition).
2. Node.js & npm (v18+ recommended).
3. PostgreSQL server (running locally or accessible via network).
   - Default DB URL: postgres://postgres:postgres@localhost:5432/directoffice

--------------------------------------------------------------------------------
1. DATABASE SETUP
--------------------------------------------------------------------------------
Run migrations using sqlx CLI (or apply SQL files manually from
Server/Backend/migrations):

  cd d:\Directoffice\Server\Backend
  # Optional if sqlx-cli is installed:
  cargo sqlx migrate run

Alternatively, connect to PostgreSQL and execute the migration files sequentially
located in:
  d:\Directoffice\Server\Backend\migrations\*.sql

--------------------------------------------------------------------------------
2. RUNNING THE SERVER (Rust / Axum)
--------------------------------------------------------------------------------
Configure environment variables if needed via .env in Server/Backend:
  DATABASE_URL=postgres://postgres:postgres@localhost:5432/directoffice
  JWT_SECRET=your-secure-secret-key-min-32-chars
  SERVER_HOST=0.0.0.0
  SERVER_PORT=8080

Start the server:
  cd d:\Directoffice\Server\Backend
  cargo run

The server listens on http://localhost:8080.
Key Endpoints:
  - GET  /api/health
  - POST /api/office/setup
  - POST /api/auth/login
  - POST /api/invitation/generate
  - POST /api/invitation/accept
  - POST /api/messaging/conversation/create
  - POST /api/messaging/message/send
  - GET  /api/messaging/message/:conversation_id
  - POST /api/voice/upload
  - GET  /api/voice/:conversation_id
  - POST /api/ai/transcribe
  - POST /api/ai/summarize
  - POST /api/ai/job
  - GET  /api/ai/job/:id
  - POST /api/task/create
  - GET  /api/task/:id
  - PUT  /api/task/:id
  - POST /api/device/register
  - POST /api/device/:device_id/revoke
  - GET  /api/device/user/:user_id

--------------------------------------------------------------------------------
3. RUNNING THE DESKTOP APP (Tauri + React + Vite)
--------------------------------------------------------------------------------
Install frontend dependencies:
  cd d:\Directoffice\Desktop\Frontend
  npm install

Run the Desktop App in Development Mode (Hot Reloading + Tauri Window):
  cd d:\Directoffice\Desktop\Frontend
  npm run tauri dev

To build the standalone release desktop executable:
  cd d:\Directoffice\Desktop\Frontend
  npm run tauri build

The built application binary will be located under:
  d:\Directoffice\target\release\directoffice-desktop.exe

--------------------------------------------------------------------------------
4. RUNNING AUTOMATED TESTS
--------------------------------------------------------------------------------
Run server unit and auth tests:
  cd d:\Directoffice\Server\Backend
  cargo test -- --nocapture

Build and check desktop client:
  cd d:\Directoffice\Desktop\Frontend
  npm run build
  cd d:\Directoffice\Desktop\Backend
  cargo check
================================================================================

