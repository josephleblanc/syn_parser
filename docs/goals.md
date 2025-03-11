# Project Goals and Roadmap

## Overview
This document outlines the step-by-step approach for building a Rust code analysis system that balances syntactic extraction with semantic relationships, specifically designed for LLM-based code generation and refactoring.

### Key Objectives:
1. **Comprehensive Code Analysis**: Extract both syntactic and semantic information
2. **Efficient Data Storage**: Optimize for fast querying and traversal
3. **LLM Integration**: Structure data for effective use with language models
4. **Extensibility**: Design for future analysis capabilities

## Roadmap

## Step 1: Expand Item Coverage

Your code already handles several important Rust items (functions, structs, enums, implementations, traits), but needs to incorporate the remaining item types:

1. **Module Structure**: ✓
   - Added `ModuleNode` to represent module hierarchy
   - Tracking imports/exports between modules to establish cross-module relationships
   - Storing module-level documentation

2. **Use Declarations & Extern Crates**: ✓
   - Created `ImportNode` to represent both use statements and extern crates
   - Established `UseRelation` edges between items and their imports
   - Now tracking which external dependencies are being used

3. **Type Aliases, Unions, and Trait Aliases**: ✓
   - Extend `TypeDefNode` enum to include these additional type definitions
   - These provide important aliasing and type relationship information

4. **Constants and Statics**: ✓
   - Add `ValueNode` to represent constants and static variables
   - Track type information and initialization expressions
   - Important for understanding program constants and global state

5. **Macros and Macro Rules**:
   - Create `MacroNode` to capture macro definitions
   - Record macro invocations as `MacroUseRelation`
   - Track macro usage across the codebase

## Step 2: Prepare Data for Analysis

### Consider integration with `sled` and `indradb`

There is no reason for me to reinvent the wheel for this project. These existing projects seem to work well together (IndraDB now [supports sled])

```rust
use sled::{Db, open};
use indradb::{Datastore, Edge, EdgeKey, Error, ModelType, Vertex, VertexKey, DataType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db: Db = open("your_db_path")?;
    let datastore = Datastore::new_sled(db);

    // Example: Inserting a node
    let node_id = VertexKey::new("Node1");
    let node = Vertex::new(node_id);
    datastore.insert_vertex(&node)?;

    // Example: Inserting an edge
    let edge_key = EdgeKey::new("Node1", "EdgeType", "Node2");
    let edge = Edge::new(edge_key);
    datastore.insert_edge(&edge)?;

    // Querying the graph
    let nodes = datastore.get_vertices()?;
    for node in nodes {
        println!("{:?}", node);
    }

    Ok(())
}
```

1. **Relationship Modeling**:
   - Implement `Calls` relationship for function invocations
   - Add data flow tracking between variables
   - Create control flow relationships

2. **Semantic Chunking for Embeddings**:
   - Create a chunking strategy for embedding functions
   - Group related items for better retrieval
   - Define boundaries for context windows

3. **Graph Query Support**:
   - Implement basic traversal methods for the graph
   - Support "find related code" queries
   - Enable "find all implementations of trait X" queries

## Step 3: Function Body Analysis (Future Work)

1. **Function Body Extraction**:
   - Create a `BlockNode` to represent code blocks
   - Extract expression and statement trees from function bodies
   - Store local variables and their scopes

2. **Control Flow Analysis**:
   - Add basic blocks as nodes with control flow edges
   - Identify loops, conditionals, and function calls
   - Track error handling paths (Result/Option unwrapping)

3. **Data Flow Tracking**:
   - Analyze variable definitions and usages
   - Track data dependencies between statements
   - Identify mutable vs immutable access patterns

## Step 4: Advanced Semantic Analysis (Future Work)

1. **Natural Code Boundaries**:
   - Individual functions with their docstrings
   - Type definitions with associated methods
   - Trait definitions with implementations

2. **Semantic Unit Identification**:
   - Group related functions by semantic purpose
   - Cluster associated types and traits
   - Identify patterns like visitor implementations, factories, or adapters

3. **Chunking Strategy**:
   - Primary chunk: Individual item with its documentation
   - Secondary chunk: Item with its immediate dependencies
   - Full context chunk: Item within its module context

## Step 5: Implementation Plan

### Phase 1: Complete Item Coverage

```rust
pub fn visit_item_mod(&mut self, module: &'ast ItemMod) {
    // Extract module information
    let module_id = self.state.next_node_id();
    let module_name = module.ident.to_string();
    
    // Process inner items if available
    if let Some((_, items)) = &module.content {
        for item in items {
            // Record relationship between module and contained items
        }
    }
    
    // Add module to graph
    self.state.code_graph.modules.push(ModuleNode {
        id: module_id,
        name: module_name,
        visibility: self.state.convert_visibility(&module.vis),
        attributes: self.state.extract_attributes(&module.attrs),
        docstring: self.state.extract_docstring(&module.attrs),
    });
    
    // Continue visiting inner items
    visit::visit_item_mod(self, module);
}
```

Similar implementations for all other Rust item types, with appropriate node structures.

### Phase 2: Function Body Analysis

```rust
fn process_fn_body(&mut self, fn_id: NodeId, body: &Block) {
    let block_id = self.state.next_node_id();
    
    // Process statements in body
    for stmt in &body.stmts {
        match stmt {
            Stmt::Local(local) => {
                // Process local variable declaration
                let var_id = self.state.next_node_id();
                let var_name = local.pat.to_token_stream().to_string();
                let type_id = local.init.as_ref().map(|(_, expr)| {
                    self.process_expression(expr)
                });
                
                // Add variable to graph and relate to function
                self.state.code_graph.variables.push(VariableNode {
                    id: var_id,
                    name: var_name,
                    type_id,
                    is_mutable: local.mutability.is_some(),
                });
                
                self.state.code_graph.relations.push(Relation {
                    source: block_id,
                    target: var_id,
                    kind: RelationKind::Declares,
                });
            },
            Stmt::Expr(expr) => {
                // Process expression statement
                self.process_expression(expr);
            },
            // Other statement types
        }
    }
    
    // Relate block to function
    self.state.code_graph.relations.push(Relation {
        source: fn_id,
        target: block_id,
        kind: RelationKind::Contains,
    });
}
```

### Phase 3: Relationship Extraction

```rust
fn process_call_expr(&mut self, expr_id: NodeId, expr: &ExprCall) {
    // Process function being called
    let func_id = self.process_expression(&expr.func);
    
    // Process arguments
    for arg in &expr.args {
        let arg_id = self.process_expression(arg);
        
        // Relate argument to call
        self.state.code_graph.relations.push(Relation {
            source: expr_id,
            target: arg_id,
            kind: RelationKind::CallArgument,
        });
    }
    
    // Add call relationship
    self.state.code_graph.relations.push(Relation {
        source: expr_id,
        target: func_id,
        kind: RelationKind::Calls,
    });
}
```

## Step 6: RAG-Specific Enhancements

1. **Semantic Indexing Strategy**:
   - Create embeddings for each declaration with its docstring
   - Store "context chunks" that include surrounding code
   - Index function implementations separately from signatures

2. **Retrieval Augmentation**:
   - For function completion queries, retrieve similar function signatures and implementations
   - For refactoring queries, retrieve the relevant type hierarchy and implementations
   - For bug fixing, retrieve error handling patterns in similar code

3. **Graph Query Support**:
   - Enable queries like "Find all implementations of trait X"
   - Support questions like "How is this type used across the codebase?"
   - Allow traversal queries like "What functions call this function?"

## Next Immediate Steps

1. **Implement Module and Use Declaration Parsing**:
   - These are foundational for understanding code organization
   - They establish important cross-file relationships

2. **Add Basic Function Body Analysis**:
   - Start with simple statement and expression extraction
   - Focus on variable declarations and function calls

3. **Expand Relationship Types**:
   - Add the `Uses` relationship to track dependencies
   - Implement `Calls` relationship for function invocations

4. **Begin Semantic Chunking**:
   - Create a chunking strategy for embedding functions
   - Test with different context window sizes

5. **Create Simple Graph Queries**:
   - Implement basic traversal methods for the graph
   - Support "find related code" queries

This plan provides a comprehensive approach to enhancing your Rust code parser for RAG purposes, focusing on both the syntactic extraction and semantic relationships that will be most valuable for code generation and refactoring tasks.

[supports sled]:https://github.com/indradb/sled?tab=readme-ov-file
