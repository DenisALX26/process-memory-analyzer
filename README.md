# Cross-Platform Process & Memory Analyzer

A high-performance, real-time system monitor written in Rust. This utility tracks process execution, thread activity, and memory allocation by interacting directly with the host operating system's low-level APIs. 

The project utilizes a multi-process architecture, decoupling the privileged data-collection engine from the user-facing CLI via Inter-Process Communication (IPC).

## Architecture

* **`pm_core`**: A shared library containing common data structures (state, memory metrics, thread counts) and serialization logic (`serde`).
* **`pm_daemon`**: A background service that extracts raw system telemetry. It handles multiple concurrent client connections by spawning dedicated threads and streaming data continuously.
* **`pm_cli`**: An interactive terminal interface that connects to the daemon, decodes the telemetry stream, and renders a live-updating resource dashboard.

## Key Technical Features

* **Cross-Platform Design:** Utilizes Rust's conditional compilation (`#[cfg(target_os = "...")]`) to isolate OS-specific logic. 
* **Linux Implementation:** Parses the `/proc` virtual filesystem (`/proc/[pid]/stat` and `statm`) directly to extract process state, RSS memory pages, and thread counts without external dependencies.
* **Inter-Process Communication (IPC):** Implements secure, real-time data streaming between the daemon and CLI using **Unix Domain Sockets**.
* **Concurrency:** The daemon uses a multi-threaded `std::thread` pool to manage independent client data streams simultaneously without blocking the main listener.

## Building and Running

**1. Start the Daemon (Data Engine)**
Run the background service. It will open the IPC socket and listen for incoming connections.
```bash
cargo run -p pm_daemon
```

**2. Start the CLI (Interactive Dashboard)**
In a separate terminal, launch the client to view the real-time memory feed.
```bash
cargo run -p pm_cli
```