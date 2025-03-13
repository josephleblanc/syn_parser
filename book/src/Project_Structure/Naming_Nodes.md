# Naming Nodes

Considering the example of `NodeId` vs `TraitId` as wrapper types around `usize` for the ID of a node.

Let's analyze this through the lens of the `goal_statement.md` and graph database requirements:

**Potential Confusion vs. Type Safety Tradeoff:**

1. **Graph Theory Perspective**  
   *In a pure graph DB*, all entities are nodes with edges. The distinction between `TraitId`/`NodeId` seems artificial.  
   *But* for a code analysis RAG system (per goal statement), we need semantic clarity more than generic graph structure.

2. **Why Distinct IDs Help**  
   - **Query Precision**  
     `MATCH (t:Trait {id: $trait_id})<-[:IMPLEMENTS]-(i:Impl)` is clearer than generic node queries
   - **Type-Safe Traversal**  
     Prevents nonsensical queries like finding a struct's fields via trait ID
   - **Performance**  
     Type-specific IDs allow optimized indices (TraitID-indexed storage vs scanning all nodes)

3. **RAG-Specific Advantages**  
   - **Embedding Accuracy**  
     LLMs benefit from explicit type distinctions when processing graph data
   - **Prompt Engineering**  
     Queries can be more specific: "Find all NodeIDs implementing TraitID X" vs vague "Connected nodes"
   - **Hallucination Prevention**  
     Hard type boundaries reduce false relationship inferences

**Mitigation Strategy**  
To balance graph purity with type safety:

```rust
// Unified graph node representation
pub enum GraphNode {
    Trait(TraitNode),
    Struct(StructNode),
    // ... others
}

// With a common ID system:
pub struct GraphNodeId {
    type_prefix: NodeType, // enum { Trait, Struct, etc }
    unique_id: usize,
}

// Conversion preserves type safety internally while enabling graph traversal
impl From<TraitId> for GraphNodeId {
    fn from(id: TraitId) -> Self {
        Self { type_prefix: NodeType::Trait, unique_id: id.0 }
    }
}
```

**Alignment with Project Goals**  

1. **Comprehensive Parsing**  
   Maintains rich type information while enabling graph queries  
2. **Relationship Tracking**  
   Explicit types prevent "false connection" hallucinations in RAG  
3. **Code Navigation**  
   Enables type-aware queries like "Find all implementors of this trait"

The temporary cognitive load of distinct IDs is outweighed by the long-term benefits for LLM-powered code analysis. The key is building an adapter layer that preserves type semantics while providing a unified graph interface.
