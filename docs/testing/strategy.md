# Code Graph Testing Strategy

## 1. Test Organization
```bash
tests/
├── parser/              # Unit tests by feature
│   ├── functions/
│   ├── structs/
│   ├── traits/
│   └── modules/
└── fixtures/            # Test case specimens
    ├── functions.rs
    ├── structs.rs
    └── traits.rs
```

## 2. Core Principles
1. **Modularity**: Isolate tests by feature domain
2. **Fixtures**: Single-responsibility test files
3. **Verification**: Standardized assertion patterns
4. **Integration**: Full-system validation

## 3. Fixture Design
```rust
// tests/fixtures/structs.rs
pub struct SimpleStruct {
    pub field: String
}

pub struct GenericStruct<T> {
    data: T
}
```

## 4. Verification Workflow
```rust
fn verify_structural_node<T: NodeVerification>(config: TestConfig<T>) {
    let graph = parse_fixture(config.fixture_path);
    let node = T::find_in_graph(&graph, config.target_id);
    
    assert_eq!(node.identifier(), config.expected.identifier);
    assert_eq!(node.visibility(), config.expected.visibility);
    assert_node_properties(node, config.expected.properties);
}
```

## 5. Integration Testing
```rust
#[test]
fn end_to_end_workflow() {
    let graph = analyze_code(Path::new("tests/data/sample.rs"));
    
    // Validate core relationships
    assert_graph_contains!(graph, StructDef, "SampleStruct");
    assert_graph_links!(graph, ImplementsRelation, "TraitA", "StructB");
    
    // Verify serialization roundtrip
    let temp_path = tempfile::NamedTempFile::new().unwrap();
    save_to_ron(&graph, &temp_path).expect("Serialization failed");
    let loaded = load_from_ron(&temp_path).expect("Deserialization failed");
    assert_graph_equivalence!(graph, loaded);
}

## 6. Maintenance Practices
1. IndexMap-based lookups (graph.rs:12)
2. Atomic ID validation (state.rs:67-72)
3. Concurrency-safe verification
4. Fixture version control
