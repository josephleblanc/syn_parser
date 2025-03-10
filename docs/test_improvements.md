# Comprehensive Test Improvement Guide

## 1. Test Organization Structure
### Current Issue: Tests are grouped by construct type but lack modular organization
### Improvement: Strict module-per-feature pattern

```bash
# Example structure
tests/
├── parser/
│   ├── functions_tests.rs
│   ├── structs_tests.rs
│   ├── traits_tests.rs
│   └── ... 
└── fixtures/
    ├── functions.rs
    ├── structs.rs
    └── ...
```

## 2. Shared Test Patterns
### Current Issue: Duplicated test setup/verification code
### Improvement: Standard verification workflow

```rust
// Example test pattern for all constructs
fn verify_node_parsing<T: NodeVerification>(config: TestConfig<T>) {
    let graph = parse_fixture(config.fixture);
    let node = T::find(&graph, config.identifier);
    
    assert_eq!(node.name(), config.expected.name);
    assert_eq!(node.visibility(), config.expected.visibility);
    T::verify_special_properties(node, config.expected.properties);
}
```

## 3. Fixture Design Principles
### Current Issue: Fixtures combine multiple features
### Improvement: Focused, single-feature fixtures

````rust
<<<<<<< SEARCH
// tests/fixtures/structs.rs
pub struct SampleStruct {
    pub field: String,
}
