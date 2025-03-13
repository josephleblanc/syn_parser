
# Refactoring ID types

**Identifying The Problem**
Looking at the code, there is indeed a potential ID type confusion issue. Here's the analysis:

1. **Current Type Definitions** (from `src/parser/nodes.rs`):
```rust
pub type NodeId = usize;
pub type TypeId = usize;  // Both IDs are aliases for usize
```

2. **Key Structures**:
```rust
// TraitNode stores its type ID
pub struct TraitNode {
    id: NodeId,          // NodeId (usize)
    // ...
    super_traits: Vec<TypeId>,  // Using TypeId (usize)
}

// ImplNode references trait via TypeId
pub struct ImplNode {
    // ...
    trait_type: Option<TypeId>,  // TypeId (usize) 
}
```

3. **Relation Creation** (from `src/parser/visitor/traits_impls.rs`):
```rust
// Creates relation using TypeId as target
self.state_mut().add_relation(Relation {
    source: impl_id,         // NodeId
    target: trait_node_id,   // NodeId (from TraitNode.id)
    kind: RelationKind::ImplementsTrait,
});
```

**The Problem**:
- Traits have both a `NodeId` (as their node ID) and a `TypeId` (for their type)
- Impl blocks store trait references as `TypeId`
- Relations use `NodeId` for both ends
- All are `usize` aliases, allowing accidental mixing

**Why This Causes Test Failures**:
When trying to match impl blocks to their traits:
1. The impl block stores a `TypeId` reference to the trait's type
2. The trait node is stored with its `NodeId`
3. Relations between them should use `NodeId` but we're using `TypeId`
4. This creates a "type system bypass" where we compare apples (type IDs) to oranges (node IDs)

**Recommended Solution**:
```rust
// Add distinct newtype wrappers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] 
pub struct TypeId(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraitId(usize);

// Update struct fields
pub struct TraitNode {
    id: TraitId,  // Instead of NodeId
    // ...
}

pub struct ImplNode {
    trait_type: Option<TraitId>,  // Instead of TypeId
    // ...
}
```

**Why This Should Be Changed**:
1. **Type Safety**: Prevent accidental mixing of ID types
2. **Clarity**: Makes code relationships explicit
3. **Maintainability**: Catches errors at compile time
4. **Domain Accuracy**: Traits are first-class entities, not just types

**Implementation Strategy**:
1. Create distinct ID types using `rustc-newtype` pattern
2. Update relation handling to use proper ID types:
```rust
Relation {
    source: ImplId,
    target: TraitId,
    kind: RelationKind::ImplementsTrait
}
```
3. Add conversion methods where intentional crossing between ID types is needed

This change would help resolve the current test failures by ensuring we're comparing trait IDs to trait IDs rather than accidentally comparing trait IDs to general type IDs. The compiler would enforce proper ID type matching through the type system.
