# Todos

A section for the immediate next steps for this project.

The current high priority tasks are related to creating more `visit_item...`
functions to more completely parse the `syn` syntax tree.

## Todos

// AI! Check on the progress of the items below.
// If any of the items are completed, then add them to the `document_history.md`

- [x] Create `visit_item_mod` and corresponding structs/enums. Handle nested modules.
- [x] Implement `visit_item_use` to handle different `UseTree` variants (simple, glob, etc.).
- [x] Create `visit_item_type` and corresponding structs/enums.
- [x] Create `visit_item_union`. Consider how to represent unions (similar to structs?).
- [x] Create `visit_item_const`.
- [x] Create `visit_item_static`. Handle mutability and potential thread-safety concerns.
- [x] Create `visit_item_foreign_mod`. Handle foreign functions and types.
  - rethink why this will be useful. If you are reading this, AI, then remind me. AI!
- [x] Decide if/how to handle verbatim tokens.
  - Decided not to handle them for now. If it becomes an obstacle later I will
  revisit.

### Improve type for NodeId

Currently there is a `NodeId` type which is a wrapper around an identifying `id` (currently a usize, should probably change to `uuid` at some point). The `NodeId` is in juxtaposition to other types like `TraitId` and `ImplId`, which represent fundamentally different kinds of data structures.

However, `NodeId` currently can hold `Struct`, `Enum`, and others. While there is some similarity around these data structures, it might be better to simply create another struct that holds a `type_prefix` and `unique_id`. See the [Naming Nodes] document for further details on implementation.

[Naming Nodes]:../Project_Structure/Naming_Nodes.md
