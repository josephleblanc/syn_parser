# Rust Code Graph Parser - Core Design Direction

## 1. Overall Project Goal
A high-performance Rust source code analyzer that incrementally builds and maintains a hybrid graph+vector representation of code semantics, optimized for integration with RAG pipelines to enable:
- Context-aware code generation/refactoring
- Cross-version dependency resolution
- Real-time documentation synthesis
- Architectural pattern detection
- Legacy code modernization

Key Differentiators:
- **Incremental Processing**: Efficient updates for active codebases
- **Hybrid Data Model**: Native graph relationships + semantic vectors
- **Hardware-Aware**: Optimized for consumer-grade ML hardware

## 2. RAG Pipeline Integration
Parser Responsibilities:
```
           +-------------------+      +-------------------+
           |  Codebase Changes |      | External Dependencies |
           +-------------------+      +-------------------+
                      |                           |
                      v                           v
            +----------------------+     +----------------------+
            | Incremental Parsing  |     | Batch Dependency Scan |
            +----------------------+     +----------------------+
                      |                           |
                      v                           v
           +----------------------+     +----------------------+
           |  Graph Structure     |     | Semantic Embeddings  |
           |  (CozoDB Relations)  |     |  (HNSW Vector DB)    |
           +----------------------+     +----------------------+
                      |                           |
                      +-----------+   +-----------+
                                  |   |
                                  v   v
                           +---------------------+
                           | Unified Query Layer |
                           | (Datalog + Vector)  |
                           +---------------------+
                                      |
                                      v
                           +---------------------+
                           | RAG Context Builder |
                           +---------------------+
                                      |
                                      v
                           +---------------------+
                           | LLM Code Generation |
                           +---------------------+
```

## 3. Core Dependency Justification

| Dependency       | Purpose                                      | Performance Rationale                     |
|------------------|---------------------------------------------|-------------------------------------------|
| **CozoDB**       | Hybrid graph+vector storage                 | Native HNSW indexing, ACID transactions   |
| **petgraph**     | In-memory graph operations                  | Fast traversal during partial updates     |
| **blake3**       | Content hashing                              | 12GB/s hashing on consumer CPUs           |
| **tokio**        | Async runtime                                | Non-blocking I/O for DB interactions      |
| **rayon**        | Parallel parsing                             | Full CPU utilization (16c/32t 9800X3D)   |
| **rust-analyzer**| Semantic analysis                            | Mature code introspection                 |

## 4. Hardware Constraints & Optimization

**System Spec Target (Your Current Hardware):**
```yaml
CPU: 16-core 9800X3D (L3 Cache Optimized)
RAM: 32GB DDR5 
Storage: PCIe 4.0 NVMe (7GB/s Read)
GPU: RTX 3060 Ti (8GB VRAM)
```

Optimization Strategy:
- **CPU-Bound Tasks** (Parsing):
  - Leverage 3D V-Cache for AST processing
  - Thread-local caching of common patterns
- **Memory Management**:
  - 8GB GPU buffer for embeddings
  - 16GB RAM for active graph (petgraph)
  - 8GB RAM for OS/DB cache
- **Storage Tiering**:
  - NVMe: Hot data (active codebase)
  - Network: Cold data (dependency tree)

## 5. Async/Concurrency Foundation

**Immediate Implementation:**
```rust
//! Architecture for 10k+ concurrent code element processing
tokio::task::spawn_blocking(move || {
    let parsed = parse_file(path); // CPU-bound
    let vectors = model.embed(parsed); // GPU-accelerated
    cozo_tx.send((parsed, vectors)) // Async channel
});

// Batch processing pipeline
rayon::scope(|s| {
    for batch in file_batches {
        s.spawn(|_| process_batch(batch, cozo_db.clone()));
    }
});
```

**Future Extensions:**
- CUDA-accelerated embedding generation
- RocksDB persistence layer
- Distributed dependency scanning
