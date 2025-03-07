# Todos

A section for the immediate next steps for this project.

The current high priority tasks are related to creating more `visit_item...`
functions to more completely parse the `syn` syntax tree.

## Todos

- [ ] Create `visit_item_mod` and corresponding structs/enums. Handle nested modules.
- [ ] Implement `visit_item_use` to handle different `UseTree` variants (simple, glob, etc.).
- [ ] Create `visit_item_type` and corresponding structs/enums.
- [ ] Create `visit_item_union`. Consider how to represent unions (similar to structs?).
- [ ] Create `visit_item_const`.
- [ ] Create `visit_item_static`. Handle mutability and potential thread-safety concerns.
- [ ] Create `visit_item_foreign_mod`. Handle foreign functions and types.
- [ ] Create `visit_item_trait_alias`.
- [ ] Decide if/how to handle verbatim tokens.
