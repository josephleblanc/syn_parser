
# AI task: Complete Item Coverage

1. **Module Structure**:
   - [x] Add `ModuleNode` to represent module hierarchy
   - [x] Track imports/exports between modules to establish cross-module relationships
   - [x] Store module-level documentation

2. **Use Declarations & Extern Crates**:
   - [x] Create `ImportNode` to represent both use statements and extern crates
   - [x] Establish `UseRelation` edges between items and their imports
   - [x] Track which external dependencies are being used

3. **Type Aliases and Unions**:
   - [x] Extend `TypeDefNode` enum to include these additional type definitions
   - [x] Add support for parsing type aliases
   - [x] Add support for parsing unions

4. **Constants and Statics**:
   - [x] Add `ValueNode` to represent constants and static variables
   - [x] Track type information and initialization expressions
   - [x] Parse visibility and attributes for constants/statics
   - [x] Add tests for constants and statics

5. **Macros and Macro Rules**:
   - [x] Create `MacroNode` to capture macro definitions
   - [x] Record macro invocations as `MacroUseRelation`
   - [x] Track macro usage across the codebase
