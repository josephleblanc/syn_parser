# AI Plans For Future `ai_next_task.md` Items

## Overall Goal

The goal of this project is to parse rust source files into data structures
that can be used to form a heterogeneous graph database. This database is
intended to be used by a Retrieval Augmented Generation (RAG) pipeline for
coding-related tasks such as code generation, code explain, code refactoring,
and documentation generation.

This project aims to achieve the goal of creating a RAG database for code by
parsing rust source files into structs that
can be used as-is or saved using `serde` to a `ron` or `json` file.

For construction of a heterogeneous graph, we identify `Item`s with [`syn`] that
will form nodes in the graph, and use static analysis to identify edge relations.

# Next Priorities

Based on your ai_todo.md, I recommend this order of implementation:

1. **Core Infrastructure**: Implement the graph database integration with sled and indradb first
2. **Chunking**: Develop the semantic chunking system
3. **Multi-file Analysis**: Enhance the parser to handle full projects
4. **Relationships**: Add deeper relationship modeling (calls, data flow)
5. **Vector Search**: Integrate with Qdrant or similar
6. **Query API**: Build the combined graph+vector query capabilities

This approach builds foundational components first before moving to more advanced features.

# Step-By-Step Plan

## 1. Integrating with Graph Databases

### Implementation Steps

1. **Add database module**:

Modify your lib.rs to integrate with rocksdb and indradb.

src/lib.rs:

 ```rust
use rocksdb::DB;
use indradb::{Datastore, Edge, EdgeKey, Vertex, VertexKey};

pub fn setup_database() -> Result<DB, Box<dyn std::error::Error>> {
    let db = DB::open_default("your_db_path")?;
    Ok(db)
}

pub fn create_nodes_and_edges(db: &DB) -> Result<(), Box<dyn std::error::Error>> {
    let datastore = indradb::Datastore::new_rocksdb(db);

    // Example: Inserting a node
    let node_id = VertexKey::new("Node1");
    let node = Vertex::new(node_id);
    datastore.insert_vertex(&node)?;

    // Example: Inserting an edge
    let edge_key = EdgeKey::new("Node1", "EdgeType", "Node2");
    let edge = Edge::new(edge_key);
    datastore.insert_edge(&edge)?;

    Ok(())
}
  ```

## 2. Semantic Chunking for Embeddings

### Implementation Steps

1. **Create chunking module**:

```rust
// src/chunking/mod.rs
pub mod strategies;

use crate::parser::nodes::Node;

pub struct Chunk {
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
    pub node_id: Option<usize>,
    pub file_path: String,
}

pub trait ChunkingStrategy {
    fn chunk(&self, content: &str, node: Option<&Node>) -> Vec<Chunk>;
      // todo
}
```

2. **Implement chunking strategies**:

```rust
// src/chunking/strategies.rs
use super::{Chunk, ChunkingStrategy};
use crate::parser::nodes::Node;

// Function-based chunking
pub struct FunctionChunker {
    pub max_chunk_size: usize,
    pub overlap: usize,
}

impl ChunkingStrategy for FunctionChunker {
    fn chunk(&self, content: &str, node: Option<&Node>) -> Vec<Chunk> {
        // Implementation...
    }
}

// Semantic window chunking
pub struct SemanticWindowChunker {
    pub window_size: usize,
    pub step_size: usize,
}

impl ChunkingStrategy for SemanticWindowChunker {
    fn chunk(&self, content: &str, node: Option<&Node>) -> Vec<Chunk> {
        // Implementation...
    }
}
```

## 3. Multi-file Analysis Support

### Implementation Steps

1. **Enhance the analyzer**:

```rust
// src/parser/mod.rs
use std::path::{Path, PathBuf};
use std::fs;
use syn::File;

pub fn analyze_directory(dir_path: &str) -> Result<CodeGraph, Box<dyn std::error::Error>> {
    let path = Path::new(dir_path);
    let mut graph = CodeGraph::new();
    
    if path.is_dir() {
        process_directory(path, &mut graph)?;
    } else {
        process_file(path, &mut graph)?;
    }
    
    Ok(graph)
}

fn process_directory(dir: &Path, graph: &mut CodeGraph) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            process_directory(&path, graph)?;
        } else if let Some(ext) = path.extension() {
            if ext == "rs" {
                process_file(&path, graph)?;
            }
        }
    }
    
    Ok(())
}

fn process_file(file_path: &Path, graph: &mut CodeGraph) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let file: File = syn::parse_str(&content)?;
    
    let mut visitor = CodeVisitor::new(file_path.to_str().unwrap_or(""), &content);
    visitor.visit_file(&file);
    
    graph.merge(visitor.into_graph());
    
    Ok(())
}
```

## 4. Relationship Modeling

### Implementation Steps

1. **Enhance relation types**:
This example is not exact, and should be adapted for our use case:

```rust
// src/parser/relations.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    // Existing types...
    Calls,          // Function calls another function
    DataFlow,       // Data flows between variables
    ControlFlow,    // Control flow between blocks
    Implements,     // Type implements trait
    DependsOn,      // Module depends on another module
}
```

2. **Enhance the visitor**:

This example is not exact, and should be adapted for our use case:

```rust
// src/parser/visitor.rs
impl<'ast> Visit<'ast> for CodeVisitor {
    // Existing implementation...
    
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        // Extract caller and callee
        if let syn::Expr::Path(path) = &*call.func {
            if let Some(func_id) = self.resolve_function_path(&path.path) {
                // Add Calls relation
                self.add_relation(Relation {
                    id: self.next_relation_id(),
                    source_id: self.current_function_id.unwrap_or(0),
                    target_id: func_id,
                    relation_type: RelationType::Calls,
                    label: "calls".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }
        
        // Visit arguments
        for arg in &call.args {
            self.visit_expr(arg);
        }
    }
}
```

## 5. Vector Database Integration

### Implementation Steps

1. **Create embeddings module**:

This example is not exact, and should be adapted for our use case:

```rust
// src/embeddings/mod.rs
use crate::chunking::Chunk;

pub trait EmbeddingModel {
    fn embed(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>>;
    fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>>;
}

pub struct OpenAIEmbedding {
    api_key: String,
    model: String,
}

impl EmbeddingModel for OpenAIEmbedding {
    // Implementation...
}
```

2. **Create vector database module**:

This example is not exact, and should be adapted for our use case:

```rust
// src/vector_db/mod.rs
use qdrant_client::prelude::*;
use crate::chunking::Chunk;

pub struct VectorDatabase {
    client: QdrantClient,
    collection_name: String,
}

impl VectorDatabase {
    pub async fn new(url: &str, collection_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize Qdrant client...
    }
    
    pub async fn add_chunks(&self, chunks: &[Chunk], embeddings: &[Vec<f32>]) -> Result<(), Box<dyn std::error::Error>> {
        // Store chunks with their embeddings...
    }
    
    pub async fn search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<(Chunk, f32)>, Box<dyn std::error::Error>> {
        // Search for similar chunks...
    }
}
```

3. **Check Added to Cargo.toml**:

```toml
[dependencies]
qdrant-client = "1.3"
tokio = { version = "1", features = ["full"] }
```

## 6. Query API

### Implementation Steps

1. **Create API module**:

```rust
// src/api/mod.rs
use async_trait::async_trait;
use crate::database::GraphDatabase;
use crate::vector_db::VectorDatabase;
use crate::embeddings::EmbeddingModel;

pub struct CodeSearchResult {
    pub content: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub score: f32,
}

#[async_trait]
pub trait CodeQueryEngine {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<CodeSearchResult>, Box<dyn std::error::Error>>;
    
    async fn find_function_calls(&self, function_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    
    async fn find_implementations(&self, trait_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    
    // Other query methods...
}

pub struct RagQueryEngine {
    graph_db: GraphDatabase,
    vector_db: VectorDatabase,
    embedding_model: Box<dyn EmbeddingModel>,
}

#[async_trait]
impl CodeQueryEngine for RagQueryEngine {
    // Implementation of the query methods...
}
```

# Deployment Options

## Option 1: Shuttle

Shuttle provides a full platform for Rust applications with built-in database support.

```rust
// src/main.rs
use shuttle_runtime::{main, SecretStore};
use shuttle_qdrant::QdrantClient;
use your_crate_name::api::RagQueryEngine;

#[shuttle_runtime::main]
async fn shuttle(
    #[shuttle_qdrant::Qdrant] qdrant: QdrantClient,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Initialize your RagQueryEngine with qdrant
    // Set up API endpoints
    // ...
}
```

## Option 2: Rig.dev

Rig provides cloud infrastructure for Rust applications with a focus on simplicity.

```toml
# rig.toml
name = "code-rag-api"
version = "0.1.0"

[deployment]
replicas = 1
resources.memory = "512Mi"

[env]
QDRANT_URL = "https://your-qdrant-instance.cloud"
```

# Suggested Enhancements to Testing Strategy

**1. Type System Depth Tests**
```rust
// tests/fixtures/types.rs
pub struct GenericStruct<T: Clone + Debug> {
    pub field: Option<Box<T>>,
}

pub enum LifetimeEnum<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

pub trait ComplexTrait<T> where T: Send {
    fn method<U>(self, param: U) -> Result<T, U> where U: Error;
}
```

**2. Cross-Reference Validation**
```rust
// tests/parser/relations_tests.rs
#[test]
fn test_trait_impl_relationships() {
    let graph = parse_fixture("traits.rs");
    
    let trait_node = find_trait_by_name(&graph, "SampleTrait").unwrap();
    let impl_blocks = find_impls_for_trait(&graph, trait_node.id);
    
    assert!(!impl_blocks.is_empty());
    assert!(impl_blocks.iter().any(|i| i.self_type == find_type_id(&graph, "SampleStruct")));
}

#[test]
fn test_module_reexports() {
    let graph = parse_fixture("modules.rs");
    
    let module = find_module_by_name(&graph, "public_module").unwrap();
    assert!(module.exports.iter().any(|id| {
        matches!(get_node_type(&graph, *id), NodeType::Struct(s) if s.name == "ReexportedStruct")
    }));
}
```

**3. Macro Edge Cases**
```rust
// tests/fixtures/macro_edge_cases.rs
#[derive(ComplexMacro!)]
struct DerivedStruct;

macro_rules! nested_macro {
    ($($inner:tt)*) => {
        macro_rules! inner_macro {
            ($($inner)*) => { /* complex pattern */ };
        }
    };
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```

# Risk Mitigation Additions

**1. Visibility Nuances**
```rust
// tests/fixtures/visibility_versions.rs
// Rust 2018 style
pub(self) struct LegacyVisibilityStruct;

// Rust 2021 style
pub(crate) mod modern {
    pub(super) struct NestedVisibility;
}
```

**2. Serialization Validation**
```rust
// tests/serialization/roundtrip_tests.rs
#[test]
fn test_struct_roundtrip() {
    let graph = parse_fixture("structs.rs");
    let temp_path = Path::new("test_output.ron");
    
    save_to_ron(&graph, temp_path).unwrap();
    let loaded = load_from_ron(temp_path).unwrap();
    
    assert_eq!(graph.defined_types.len(), loaded.defined_types.len());
    let original_struct = find_struct_by_name(&graph, "SampleStruct").unwrap();
    let loaded_struct = find_struct_by_name(&loaded, "SampleStruct").unwrap();
    assert_eq!(original_struct.fields, loaded_struct.fields);
}

#[test]
fn test_relationship_persistence() {
    let graph = parse_fixture("impls.rs");
    let temp_path = Path::new("relationships.ron");
    
    save_to_ron(&graph, temp_path).unwrap();
    let loaded = load_from_ron(temp_path).unwrap();
    
    assert_eq!(graph.impls.len(), loaded.impls.len());
    assert_eq!(graph.relations.len(), loaded.relations.len());
}
```

**3. Performance Benchmarks**
```rust
// benches/parser_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use syn_parser::parser;

fn large_file_benchmark(c: &mut Criterion) {
    c.bench_function("parse_large_codebase", |b| {
        b.iter(|| {
            parser::analyze_code(black_box(Path::new("benches/large_fixture.rs")))
        });
    });
}

criterion_group!(benches, large_file_benchmark);
criterion_main!(benches);
```

# Implementation Advice

1. **Test Organization**
```bash
mkdir -p tests/fixtures tests/parser tests/serialization benches
```

2. **Add Testing Dependencies**
```bash
cargo add --dev criterion test-case
```

3. **Incremental Implementation**
```rust
// Start with type system tests
#[test]
fn test_generic_params() {
    let graph = parse_fixture("types.rs");
    let generic_struct = find_struct_by_name(&graph, "GenericStruct").unwrap();
    
    assert_eq!(generic_struct.generic_params.len(), 1);
    if let GenericParamKind::Type { bounds, .. } = &generic_struct.generic_params[0].kind {
        assert!(bounds.iter().any(|tid| matches!(
            graph.type_graph[*tid].kind,
            TypeKind::TraitBound { name: "Clone" }
        )));
    }
}

// Then add relationship verification
#[test]
fn test_type_references() {
    let graph = parse_fixture("structs.rs");
    let field_type = &graph.type_graph[struct_node.fields[0].type_id];
    
    if let TypeKind::Named { path, .. } = &field_type.kind {
        assert_eq!(path, &["String"]);
    }
}
```
