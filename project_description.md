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

## Parser Module
**Path:** `src/parser/mod.rs`  
**Purpose:** Central coordination point for code parsing and analysis infrastructure

### Dependencies
- Internal Submodules:
  - `graph`: Core graph structure implementation
  - `visitor`: AST traversal and analysis implementation
  - `types`: Type system representation
- External Crates:
  - `syn`: Rust syntax parsing
  - `indexmap`: Preserved insertion order for analysis

### Primary Exports
- `CodeGraph`: Central data structure containing parsed code relationships
- `TypeId`: Opaque identifier for type system entities
- `analyze_code`: Main entry point for file analysis

### Integration Points
- Consumed by:
  - Library root (`lib.rs`) as primary export
  - Serialization modules for graph transformation
- Coordinates between:
  - AST Visitor pattern implementation
  - Graph construction logic
  - Type system resolution

---

### Graph Structure Implementation
**Path:** `src/parser/graph.rs**  
**Purpose:** Central data structure storing all parsed code relationships

#### Core Components
- `CodeGraph` struct fields:
  - `functions`: IndexMap of NodeId to FunctionNode (preserving declaration order)
  - `defined_types`: Aggregate of struct/enum/union/alias definitions
  - `type_graph`: Collection of all type references with relationships
  - `impls`: Implementation blocks grouped by self-type
  - `traits`: Public trait definitions with method signatures
  - `relations`: Directed edges between nodes (inheritance, calls, etc)

#### Key Relationships
- Uses `NodeId` from `graph_ids.rs` as primary identifier
- Contains `Relation` enum from `relations.rs`
- Stores concrete node types from `nodes.rs`
- Built by visitor pattern in `visitor/` module

#### Serialization
- Derives `Serialize`/`Deserialize` for RON persistence
- Maintains insertion order for deterministic output

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
