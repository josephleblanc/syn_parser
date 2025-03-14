# Project Structure Documentation

## Core Library Crate
**Path:** `src/lib.rs`  
**Purpose:** Primary entry point exposing public API for code analysis and graph generation

### Dependencies
- Internal Modules:
  - `parser`: Code parsing and AST traversal infrastructure
  - `serialization`: Graph serialization implementations
  - `config`: (Nascent) Configuration management stubs

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

## Configuration Module
**Path:** `src/config/mod.rs`  
**Purpose:** Placeholder for future configuration management system

### Current State
- Empty module file (0 exports)
- Adjacent `options.rs` contains no implementation
- Not yet integrated with other components

### Configuration Options (Planned)
**Path:** `src/config/options.rs`  
**Purpose:** Intended to define configuration structures

#### Planned Structure
```rust
// Expected to contain:
pub struct ParserConfig {
    pub preserve_comments: bool,
    pub detect_macros: bool,
}

pub struct SerializationConfig {
    pub format: OutputFormat, // enum { Json, Ron }
    pub pretty_print: bool,
}
```

#### Required Connections
- Would need to integrate with:
  - `parser/visitor/state.rs` for analysis parameters
  - `serialization/mod.rs` for output formatting
  - Future CLI arguments in `main.rs.back`

### Integration Needs
- Requires connection to:
  - CLI arguments (future main binary)
  - Visitor pattern configuration
  - Serialization format selection
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
**Purpose:** (Current placeholder) Foundation for error type definitions

### Current State
- Contains only a TODO comment placeholder
- Not yet integrated with other components
- Missing concrete error type implementations

### Immediate Integration Needs
- Define core error enum matching foundational types:
  - `NodeId`, `TypeId` references from parser
  - Relation types from graph module
- Connect to visitor pattern error handling
- Establish error conversion traits for serialization

### Critical Dependencies
- Requires integration with:
  - `parser/visitor/state.rs` (graph construction errors)
  - `serialization/mod.rs` (serialization failures)
  - `parser/utils.rs` (parsing validation)

---
