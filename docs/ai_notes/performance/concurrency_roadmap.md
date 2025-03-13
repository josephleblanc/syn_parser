# Concurrency & Async Performance Roadmap

## Immediate Low-Effort High-Impact Changes

### 1. Atomic ID Generation (Thread-Safe Foundation)
```rust
// src/parser/visitor/state.rs
<<<<<<< SEARCH
    pub next_node_id: usize,
    pub next_trait_id: usize,
    pub next_type_id: usize,
