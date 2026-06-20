# Google Suite RS 🚀

![Crates.io Version](https://img.shields.io/crates/v/google_suite_rs?style=for-the-badge&color=orange)
![Rust](https://img.shields.io/badge/rust-stable-blue?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)
![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-brightgreen?style=for-the-badge)

A unified, type-safe Rust interface for Google Flow, Gemini Spark, and AI Mode.

### Project Breakdown

**What is this about?**
`Google Suite RS` is a Rust-based library designed to act as a bridge to Google’s advanced AI services. It packages three distinct Google AI paradigms into one cohesive repository.

**What does it do?**
It provides developers with a modular API to trigger generative AI workflows, deploy and maintain stateful, long-running agents, and scrape or aggregate data across the web using Google's infrastructure. It abstracts the complex network requests, authentication (`auth_interceptor.rs`), rate limiting (`rate_limiter.rs`), and data parsing into native Rust structs and asynchronous functions.

**What problems does it solve?**
* **Fragmentation:** Instead of managing separate HTTP clients, authentication flows, and data schemas for different Google AI tools, it centralizes them into one standard crate.
* **Type Safety & Performance:** By wrapping these APIs in Rust, it ensures memory safety, robust error handling, and strict type definitions for the JSON payloads coming to and from Google's servers. This is particularly crucial for systems that require high concurrency, low latency, and zero tolerance for runtime type errors.
* **Complex Orchestration:** It simplifies the orchestration between different AI models (e.g., having an AI Mode agent aggregate web data, and then passing that data to Flow for a generative task).
* 

## 🧠 Overview

Google Suite RS abstracts Google's core AI architectures into a modular, high-performance library natively built for Rust environments. 

### System Architecture Tiers

The library uses a modular design to abstract Google's three primary compute tiers:

1. **Flow:** Handles generative compute tasks.
2. **Spark:** Manages long-running persistent agents.
3. **AI Mode:** Aggregates multi-source web data.

---

## 📦 Installation

Add the following to your `Cargo.toml` dependencies block:

```toml
[dependencies]
google_suite_rs = "0.1.0"

cargo install --path .
