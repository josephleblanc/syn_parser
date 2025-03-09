# Plan

Here's a step-by-step approach that balances syntactic extraction with semantic relationships useful for an LLM-based code generation and refactoring system.

## Step 1: Expand Item Coverage ✓

Your code already handles several important Rust items (functions, structs, enums, implementations, traits), and now also includes:

1. **Module Structure**: ✓
   - Added `ModuleNode` to represent module hierarchy
   - Tracking imports/exports between modules to establish cross-module relationships
   - Storing module-level documentation

2. **Use Declarations & Extern Crates**: ✓
   - Created `ImportNode` to represent both use statements and extern crates
   - Established `UseRelation` edges between items and their imports
   - Now tracking which external dependencies are being used

3. **Type Aliases, Unions, and Trait Aliases**:
   - Extend `TypeDefNode` enum to include these additional type definitions
   - These provide important aliasing and type relationship information

4. **Constants and Statics**:
   - Add `ValueNode` to represent constants and static variables
   - Track type information and initialization expressions
   - Important for understanding program constants and global state

5. **Macros and Macro Rules**:
   - Create `MacroNode` to capture macro definitions
   - Record macro invocations as `MacroUseRelation`
   - This is critical for understanding Rust's meta-programming features

## Step 2: Enhance Function and Block Analysis

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

## Step 3: Relationship Modeling

1. **Direct Relationships**:
   - Function calls (`CallsRelation`)
   - Type containment (`ContainsRelation`)
   - Trait implementations (`ImplementsRelation`)
   - Type conversions/coercions (`ConvertToRelation`)

2. **Semantic Relationships**:
   - Function similarity (based on parameter/return types)
   - Type hierarchy relationships
   - Producer/consumer relationships (functions producing values consumed by others)

3. **Dependency Graph**:
   - Add `DependsOn` relationships to show item interdependencies
   - Create a directed acyclic graph (DAG) of dependencies
   - Useful for understanding code organization

## Step 4: Semantic Chunking for Embeddings

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

1. **Complete Remaining Item Types**:
   - Implement Type Aliases, Unions, and Trait Aliases parsing
   - Add Constants and Statics support
   - Implement Macros and Macro Rules parsing

2. **Add Basic Function Body Analysis**:
   - Start with simple statement and expression extraction
   - Focus on variable declarations and function calls
   - Create a `BlockNode` to represent code blocks

3. **Expand Relationship Types**:
   - Implement `Calls` relationship for function invocations
   - Add data flow tracking between variables
   - Create control flow relationships

4. **Begin Semantic Chunking**:
   - Create a chunking strategy for embedding functions
   - Test with different context window sizes
   - Group related items for better retrieval

5. **Create Simple Graph Queries**:
   - Implement basic traversal methods for the graph
   - Support "find related code" queries
   - Enable "find all implementations of trait X" queries

This plan provides a comprehensive approach to enhancing your Rust code parser for RAG purposes. You've already completed the first major milestone by implementing module structure and use declarations. The next steps will focus on completing the remaining item types and enhancing the semantic relationships that will be most valuable for code generation and refactoring tasks.
