# ⚡ Veloxis (Salvo + Leptos Full-Stack Rust)

A modern, high-performance, full-stack web application built entirely in **100% Pure Rust**. This project bypasses JavaScript completely by pairing the ergonomic **Salvo** backend framework with the blazing-fast, fine-grained reactive **Leptos** frontend framework compiling to WebAssembly (Wasm).

## 🚀 Key Features

* **Zero JavaScript:** Built entirely in Rust from the server logic down to the browser DOM.
* **Fine-Grained Reactivity:** Powered by Leptos Signals—no heavy Virtual DOM overhead.
* **Asynchronous Backend:** Powered by Salvo, Hyper, and Tokio for maximum network throughput.
* **End-to-End Type Safety:** Share data models natively between your backend API and frontend UI.
* **Server-Side Rendering (SSR) & Hydration:** Instant page loads with full SEO support, seamlessly activated by WebAssembly.

---

## 📂 Project Structure

This repository is organized as a Cargo workspace:

```text
├── backend/       # Salvo web server (API routes, SSR logic, static assets)
├── frontend/      # Leptos UI application (compiles down to WebAssembly)
├── shared/        # Shared data structures used by both backend and frontend
└── Cargo.toml     # Cargo workspace configuration
```

---

## 🛠️ Prerequisites

To build and run this application locally, you need the Rust toolchain installed alongside the WebAssembly compilation target.

1. **Install Rust:** [rustup.rs](https://rustup.rs)
2. **Add the Wasm Target:**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **Install Trunk** (The premier Rust/Wasm bundler tool):
   ```bash
   cargo install --locked trunk
   ```

---

## 💻 Development & Building

Follow these steps to compile the frontend and launch your full-stack application server.

### 1. Build the Frontend (Wasm)
Navigate to the frontend directory and build the client assets:
```bash
cd frontend
trunk build --release
```
*This compiles your Rust UI into an optimized `.wasm` binary and generates the JavaScript glue code inside `frontend/pkg/`.*

### 2. Run the Backend (Salvo)
Return to the root workspace directory and launch the web server:
```bash
cd ..
cargo run -p backend
```

### 3. Open the Application
Open your browser and navigate to:
```text
http://127.0.0.1:5800
```

---

## 🤝 Architecture Flow

1. **Data Definition:** Define a `struct` in `shared/src/lib.rs` with `#[derive(Serialize, Deserialize)]`.
2. **Server (Salvo):** Processes incoming network requests, interacts with your database, and uses the shared struct to render initial HTML on the server.
3. **Client (Leptos):** Downloads the `.wasm` file, hydates the HTML, and handles lightning-fast client-side updates instantly when signals change.

---

## 📄 License

This project is licensed under the MIT License.
