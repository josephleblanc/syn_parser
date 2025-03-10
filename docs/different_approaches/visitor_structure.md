### Alternative Approach: Visitor Composition Breakup
Split into vertical slices with dedicated visitor traits:

1. **Core Visitor Trait** (`visitor/mod.rs`)
```rust
pub trait CodeVisitor {
    fn visit_annotated_item(&mut self, item: &Item); // Main dispatch point
}
```

2. **Specialized Visitors** (`visitor/specialized/`)
```rust
// visitor/specialized/functions.rs
pub trait FunctionVisitor {
    fn visit_function(&mut self, func: &ItemFn);
    fn process_parameters(&mut self, sig: &Signature);
}

// visitor/specialized/types.rs
pub trait TypeVisitor {
    fn process_generic_param(&mut self, param: &GenericParam);
    fn handle_type_bound(&mut self, bound: &TypeParamBound);
}
```

### Key Recommendations:
1. **State Management First**
- Extract all ID generation and type management to `state.rs`
- Make state mutations explicit through defined interfaces

2. **Type System Isolation**
- Build a strong abstraction boundary around type processing
- Use the visitor pattern recursively for complex types

3. **Relation Builder Pattern**
- Create a dedicated relation manager that handles:
  ```rust
  pub struct RelationBuilder<'a> {
      state: &'a mut VisitorState,
      current_scope: NodeId,
  }
  ```

4. **AST Processing Layers**
```rust
pub mod processing {
    pub mod declarations;  // Function/struct/trait declarations
    pub mod expressions;   // Expression analysis
    pub mod relationships; // Relationship detection
}
```

