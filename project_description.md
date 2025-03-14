# Project Structure Documentation

## Core Library Crate
**Path:** `src/lib.rs`  
**Purpose:** Primary entry point exposing public API for code analysis and graph generation

### Dependencies
- Internal Modules:
  - `parser`: Code parsing and AST traversal infrastructure
  - `serialization`: Graph serialization implementations

### Primary Exports
- `analyze_code`: Main entry function for code analysis
- `CodeGraph`: Central data structure representing code relationships
- `save_to_ron`: Primary serialization method

### Integration Points
- Consumed by:
  - Binary targets (via `main.rs.back` prototype)
  - External integration tests
- Exports foundational types used throughout:
  - Graph node/relation identifiers
  - Serialization formats

---

## Foundational Types (Candidate Exports)
**Potential Core Primitives:**
- `GraphNodeId`: Composite identifier combining node type and unique ID
- `NodeId`: Opaque identifier for graph nodes
- `TraitId`: Specialized identifier for trait definitions
- `TypeId`: Unique identifier for type system entities
- `Relation`: Enum representing various code relationships

---

## Error Handling Infrastructure
**Path:** `src/error.rs`  
**Purpose:** Centralized error types and handling for code analysis pipeline

### Key Data Structures
- `AnalysisError` (Planned):
  - Purpose: Unified error type for parsing/analysis failures
  - Planned variants:
    - `SyntaxError`: Wraps parser-level failures
    - `TypeResolution`: Failed type lookups/inferences
    - `CircularDependency`: Invalid graph relationships

### Error Handling Strategy
- Planned error propagation:
  - Use `thiserror` crate for explicit variant definitions
  - Implement `From` trait for foreign error types
  - Context-aware error reporting with source chains

### Integration Points
- Cross-cutting concern used by:
  - Parser modules
  - Visitor implementations
  - Serialization components
- Will unify error reporting across:
  - CLI interface (main.rs.back)
  - Library consumers
  - Test assertions

---
