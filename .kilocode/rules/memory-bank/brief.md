# Project Brief: ClewdR

## 1. Project Purpose and Goals

**ClewdR** is a production-grade, high-performance proxy server designed for Large Language Models (LLMs), specifically **Claude** (Claude.ai, Claude Code) and **Google Gemini** (AI Studio, Vertex AI). 

The primary goal of the project is to provide an extremely fast, resource-efficient, and reliable proxy solution with an enterprise-grade architecture and a user-friendly experience. It aims to deliver a 10x performance improvement over traditional script-based proxies while maintaining a minimal memory footprint (single-digit MBs in production).

## 2. Technologies and Frameworks

- **Backend Language**: Rust
- **Core Framework**: Tokio + Axum for asynchronous, high-throughput request handling.
- **Frontend**: React-powered web console for real-time monitoring and management.
- **Caching**: Moka for intelligent, in-memory caching.
- **HTTP Client**: `wreq` with Chrome-level browser fingerprinting for seamless API access.

## 3. Key Features

- **High Performance**: Built with Rust for maximum speed and low resource consumption, capable of handling thousands of requests per second.
- **Full-Featured Web UI**: A React-based console for real-time monitoring, multi-language support, hot-reloading of configurations, and visual management of Cookies and API Keys.
- **Enterprise-Grade Architecture**: Features event-driven design, multi-threaded processing, and intelligent resource management.
- **Intelligent Resource Management**: Includes smart Cookie rotation, API key health monitoring, rate-limiting with exponential backoff, and connection pooling.
- **Universal Compatibility**: Deploys as a single, zero-dependency binary across Windows, macOS, Linux, and Android. Also Docker-ready.
- **Comprehensive Protocol Support**:
    - **Claude**: Supports Claude.ai, Claude Code, system prompt caching, extended thinking mode, and image attachments.
    - **Google Gemini**: Supports AI Studio and Vertex AI with OAuth2 authentication.
    - **API Compatibility**: Provides endpoints compatible with both native (Claude, Gemini) and OpenAI API formats.

## 4. Installation and Setup

1.  **Download**: Get the latest release binary for the target platform (Windows, Linux, macOS, etc.).
2.  **Execute**: Run the binary directly.
3.  **Access Web UI**: Open a browser to `http://127.0.0.1:8484`.
4.  **Login**: Use the `Web Admin Password` displayed in the console.
5.  **Configure**: Add Claude Cookies or Gemini API keys through the web interface.
6.  **Connect**: Point applications to the appropriate API endpoints provided by ClewdR.