# Tech Stack and Setup: ClewdR

## 1. Core Technologies

- **Language**: **Rust (2024 Edition)**
  - Chosen for its performance, memory safety, and concurrency capabilities, which are essential for a high-throughput proxy server.

- **Asynchronous Runtime**: **Tokio**
  - The entire application is built on Tokio, enabling it to handle thousands of concurrent I/O-bound tasks (like network requests) efficiently with a small number of system threads.

- **Web Framework**: **Axum**
  - A modular and ergonomic web framework built by the Tokio team. It integrates seamlessly with the Tokio ecosystem and provides a robust foundation for building web services.

- **HTTP Client**: **wreq**
  - Used for making outgoing requests to the Claude and Gemini APIs. It's configured with Chrome-level browser fingerprinting to mimic real browser requests, which is crucial for interacting with web-based APIs.

- **Actor Framework**: **Ractor**
  - Used to implement the actor model for state management. The `CookieActor` and `KeyActor` are central to managing shared, mutable state (the pools of Cookies and API Keys) in a concurrent environment, preventing race conditions.

- **Frontend**: **React + TypeScript + Vite**
  - The web UI is a modern single-page application (SPA) built with React and TypeScript, providing a responsive and type-safe development experience. Vite is used as the build tool for its fast development server and optimized production builds.

## 2. Development Environment

### a. Backend Setup

1.  **Install Rust**: Get the Rust toolchain, including `rustc` and `cargo`, from [rust-lang.org](https://www.rust-lang.org/).
2.  **Build**: Navigate to the project root and run `cargo build`. For an optimized production build, use `cargo build --release`.
3.  **Run**: Execute the binary from the `target/debug/` or `target/release/` directory: `./clewdr`.

### b. Frontend Setup

1.  **Install Node.js and pnpm**: The frontend uses `pnpm` as its package manager.
2.  **Navigate to Frontend Directory**: `cd frontend`
3.  **Install Dependencies**: `pnpm install`
4.  **Run Dev Server**: `pnpm dev`. This will start the Vite development server, which typically proxies API requests to the running backend on port `8484`.

## 3. Key Dependencies (from `Cargo.toml`)

- `serde` & `serde_json`: For serialization and deserialization of JSON data, used in almost all API interactions.
- `tracing` & `tracing-subscriber`: For structured, asynchronous-aware logging.
- `moka`: An in-memory caching library used for features like system prompt caching.
- `clap`: For parsing command-line arguments.
- `sea-orm`: (Optional feature) An asynchronous and dynamic ORM used for database persistence when the `db-sqlite`, `db-postgres`, or `db-mysql` features are enabled.