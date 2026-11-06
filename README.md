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

