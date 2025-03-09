
# AI task: Complete Item Coverage

1. **Module Structure**:
   - [x] Add `ModuleNode` to represent module hierarchy
   - [x] Track imports/exports between modules to establish cross-module relationships
   - [x] Store module-level documentation

2. **Use Declarations & Extern Crates**:
   - [x] Create `ImportNode` to represent both use statements and extern crates
   - [x] Establish `UseRelation` edges between items and their imports
   - [x] Track which external dependencies are being used

3. **Type Aliases, Unions, and Trait Aliases**:
   - [ ] Extend `TypeDefNode` enum to include these additional type definitions
   - [ ] Add support for parsing type aliases
   - [ ] Add support for parsing unions
   - [ ] Add support for parsing trait aliases

4. **Constants and Statics**:
   - [ ] Add `ValueNode` to represent constants and static variables
   - [ ] Track type information and initialization expressions
   - [ ] Parse visibility and attributes for constants/statics

5. **Macros and Macro Rules**:
   - [ ] Create `MacroNode` to capture macro definitions
   - [ ] Record macro invocations as `MacroUseRelation`
   - [ ] Track macro usage across the codebase
