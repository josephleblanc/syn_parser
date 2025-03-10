# Test Improvements Guide

## 1. Test Organization Improvements
```bash
mkdir -p tests/parser
```

1. Create dedicated test modules for different impl types:
```rust
// In tests/parser/impls_tests.rs
mod inherent_impls {
    // Test cases for impl blocks without traits
}

mod trait_impls {
    // Test cases for trait implementations
}

mod edge_cases {
    // Test special scenarios like blanket impls, where clauses, etc
}
```

## 2. Shared Verification Logic

2. Add a config struct and verification function:
```rust
struct ImplTestConfig {
    fixture: &'static str,
    type_name: &'static str,
    trait_name: Option<&'static str>,
    expected_methods: Vec<&'static str>,
    generic_params: Option<Vec<&'static str>>,
}

fn verify_impl_block(config: ImplTestConfig) {
    // Shared assertion logic using the config
}
```

## 3. Enhanced Test Helpers

3. Add these helpers to tests/common/mod.rs:
```rust
pub fn find_impls_for_type<'a>(graph: &'a CodeGraph, type_name: &str) -> Vec<&'a ImplNode> {
    // Returns all impl blocks for a type
}

pub fn find_trait_impls<'a>(
    graph: &'a CodeGraph,
    trait_name: &str,
    type_name: &str
) -> Vec<&'a ImplNode> {
    // Finds specific trait implementations
}
```

## 4. Edge Case Coverage

4. Add test stubs for complex scenarios:
```rust
#[test]
fn test_impl_with_where_clause() {
    // Test impl blocks with complex where clauses
}

#[test]
fn test_blanket_impl() {
    // Test blanket implementations (impl<T> ... for T)
}

#[test] 
fn test_impl_for_foreign_type() {
    // Test impls for types from other modules/crates
}
```

## 5. Test Documentation

5. Add purpose comments to each test module:
```rust
/// Tests for inherent implementations (impl blocks without traits)
mod inherent_impls {
    //...
}

/// Tests for trait implementations (explicit trait impls)
mod trait_impls {
    //...
}

/// Tests for non-trivial implementation scenarios
mod edge_cases {
    //...
}
```

## 6. Suggested Implementation Order

1. Create helper functions first
2. Add test configuration structure
3. Implement shared verification logic
4. Reorganize existing tests into modules
5. Add new edge case tests
6. Update documentation and assertions

Run tests after each step:
```bash
cargo test --test impls_tests
```
