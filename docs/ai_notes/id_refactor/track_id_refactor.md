# ID Type Refactor Tracking

## Files Requiring Changes

1. `src/parser/nodes.rs` ✔️
   - Node ID type definitions ✔️
   - Type conversions ✔️  
   - Core ID initialization ✔️
   - Serialization derives ✔️
   - TraitNode/ImplNode struct fields (Phase 2)
   - Implemented From/Into traits ✔️

2. `src/parser/relations.rs` ✔️
   - Relation struct fields ✔️
   - RelationKind variants handling ✔️
   - Temporary conversion support ✔️
   - Type imports ✔️

3. `src/parser/visitor/state.rs` ✔️
   - ID generation methods ✔️
   - Type system operations ✔️
   - Type conversions ✔️
   - CodeGraph storage fields (Phase 3)

3. `src/parser/visitor/traits_impls.rs`
   - Trait-impl relationship handling
   - Type ID vs Trait ID comparisons

4. `src/parser/visitor/state.rs`
   - ID generation methods
   - CodeGraph storage fields
   - Type system operations

5. `tests/parser/impls_tests.rs`
   - Test assertions
   - Node lookup logic

## Key Data Structure Relationships

```mermaid
graph TD
    TraitNode -->|has| TraitID
    ImplNode -->|references| TraitID
    TypeNode -->|has| TypeID
    Relation -->|links| NodeID
    Relation -->|links| TraitID
    CodeGraph -->|stores| TraitNode
    CodeGraph -->|stores| ImplNode
```

## Critical Methods to Update

1. `VisitorState::next_node_id()`
2. `VisitorState::next_type_id()` 
3. `TypeOperations::get_or_create_type()`
4. `ImplVisitor::process_impl()`
5. `TraitVisitor::process_trait()`

## Refactor Strategy

1. **Add New ID Types First**
2. **Update Struct Fields Gradually**
3. **Modify Relation Handling**
4. **Adjust Test Assertions Last**
5. **Use Compiler Errors as Guide**

## Preserving Functionality

- Maintain existing serialization format
- Keep ID generation sequential
- Add explicit conversions where needed
- Use type aliases during transition

## Testing Approach

1. Focus on `impls_tests.rs` first
2. Verify trait-impl relationships
3. Check cross-references in serialized output
4. Use `cargo check` after each file change

## Relevant Instructions from refactor_test_progress.md

- "Make small, focused changes"
- "Validate after each fix" 
- "Check for compiler errors with `cargo check 2>&1 | rg -A 8 E0`"
- "Use failing tests as specification"
