I'll analyze the documents and create a phased refactor plan. Based on the codebase state and ID confusion issues described, here's the step-by-step strategy:

PHASE 1: FOUNDATIONAL TYPE SAFETY
---------------------------------
1. **Implement Core ID Types** (`nodes.rs`, `types.rs`)
```rust
// New: src/parser/nodes.rs
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(usize);

// New: src/parser/types.rs 
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeId(usize);

// Keep existing TypeId usage temporarily
pub type LegacyTypeId = usize;
```

2. **Update ID Generation** (`state.rs`)
```rust
// In VisitorState
fn next_node_id(&mut self) -> NodeId {
    let id = NodeId(self.next_node_id);
    self.next_node_id += 1;
    id
}

fn next_type_id(&mut self) -> TypeId {
    let id = TypeId(self.next_type_id);
    self.next_type_id += 1;
    id
}
```

3. **Safe Conversion Helpers** (`types.rs`)
```rust
impl TypeId {
    pub fn as_node_id(self) -> Option<NodeId> {
        // Temporary bridge for initial refactor
        Some(NodeId(self.0))
    }
}
```

PHASE 2: TRAIT-SPECIFIC IDENTIFIERS
-----------------------------------
1. **Add TraitId Type** (`nodes.rs`)
```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraitId(usize);
```

2. **Update TraitNode Structure** (`nodes.rs`)
```rust
pub struct TraitNode {
    pub id: TraitId,  // Changed from NodeId
    // ... other fields unchanged
}
```

3. **Impl-Trait Relation Handling** (`traits_impls.rs`)
```rust
// Updated relation creation
self.state_mut().add_relation(Relation {
    source: impl_id.into(), // NodeId -> TraitId conversion
    target: trait_node_id,
    kind: RelationKind::ImplementsTrait,
});
```

PHASE 3: RELATION SYSTEM OVERHAUL
---------------------------------
1. **Strengthen Relation Types** (`relations.rs`)
```rust
pub struct Relation {
    pub source: RelationSource,
    pub target: RelationTarget,
    pub kind: RelationKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationSource {
    Node(NodeId),
    Trait(TraitId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationTarget {
    Type(TypeId),
    Trait(TraitId),
}
```

2. **Type System Audit Points** (critical checks):
- `process_impl()`: Verify TraitId/TypeId separation
- `process_trait()`: Ensure TraitId generation
- Test helpers: Update ID comparison logic

PHASE 4: TEST SYSTEM ADAPTATION
--------------------------------
1. **Test Helper Updates** (`common/mod.rs`)
```rust
// Updated find_trait_by_name
pub fn find_trait_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a TraitNode> {
    graph.traits.iter()
        .chain(graph.private_traits.iter())
        .find(|t| t.name == name)
}
```

2. **Assertion Helpers** (`impls_tests.rs`)
```rust
fn assert_relation_exists(
    graph: &CodeGraph,
    source: impl Into<RelationSource>,
    target: impl Into<RelationTarget>,
    kind: RelationKind
) {
    // New type-aware assertion
}
```

SAFETY CHECKS & VALIDATION STEPS
--------------------------------
1. **Incremental Compilation Checks**
```bash
# After each file change:
cargo check 2>&1 | grep -A 8 "error[E0308]"
```

2. **Test Preservation Sequence**
```text
1. test_impl_for_struct
2. test_generic_impl_for_trait
3. test_find_impl_by_name
4. test_generic_impl_for_struct
```

3. **Serialization Validation**
```rust
// Add temporary debug output
let ron = save_to_ron(&graph);
std::fs::write("debug_graph.ron", ron);
```

REFACTOR TIMELINE (EST. 3-4 HOURS)
-----------------------------------
```mermaid
gantt
    title ID Refactor Timeline
    dateFormat  HH-mm
    section Phase 1
    Core ID Types       : 00-00, 40min
    ID Generation       : 00-40, 30min
    Initial Compile     : 01-10, 20min
    
    section Phase 2
    Trait ID System     : 01-30, 45min
    Relation Conversion : 02-15, 30min
    Test Updates        : 02-45, 45min
    
    section Validation
    Serialization Check : 03-30, 20min
    Full Test Run       : 03-50, 20min
```

CRITICAL WATCH POINTS
---------------------
1. **TypeId Leakage**  
   Watch for TypeId usage where TraitId should be used after Phase 2

2. **Serialization Breakage**  
   Verify RON output structure remains compatible

3. **Test False Positives**  
   Ensure tests actually fail when relationships break

4. **Visibility Handling**  
   Maintain restricted visibility path tracking during ID changes

Let's start with Phase 1, file by file. Would you like me to generate the first concrete code changes for the NodeId/TypeId newtype implementation?
