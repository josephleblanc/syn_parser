
# AI task: Complete Item Coverage

1. **Module Structure**:
   - [x] Add `ModuleNode` to represent module hierarchy
   - [x] Track imports/exports between modules to establish cross-module relationships
   - [ ] Store module-level documentation

2. **Use Declarations & Extern Crates**:
   - [ ] Create `ImportNode` to represent both use statements and extern crates
   - [ ] Establish `UseRelation` edges between items and their imports
   - [ ] This helps track which external dependencies are being used

