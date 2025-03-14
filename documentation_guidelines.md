Documentation should:

## Core Principles
1. **Accuracy First** - Precisely reflect current implementation state
2. **Code-Centric** - Reference actual file paths and line numbers
3. **Change Tracking** - Highlight discrepancies between planned/actual code
4. **Dependency Mapping** - Explicitly connect components through imports and calls

## Technical Documentation
5. **Concrete Examples** - Show real function signatures with error handling
6. **Implementation Details**:
   - Data structure memory ownership
   - Serialization/deserialization boundaries
   - Type conversion traits
7. **Versioning** - Track RON format requirements and compatibility
8. **Validation** - Document constraint checks and error conditions

## Visual Documentation
9. **Architecture Diagrams** - Use Mermaid.js for system overviews
10. **Flow Charts** - Illustrate key workflows (type resolution, node creation)
11. **Interaction Matrices** - Tabular views of cross-component relationships

## Consistency Checks
12. **Pattern Tracking**:
   - Consistent use of derive macros
   - ID generation strategies
   - Visibility handling
13. **Deviation Log** - Catalog intentional pattern breaks with justification
14. **Test/Prod Parity** - Flag test-only code paths needing productionization

## Implementation Focus Areas
- Visitor pattern traversal order and state mutations
- Type system unification process
- Graph storage backends (CozoDB vs RON)
- Macro expansion and attribute processing
- Error propagation through analysis pipeline

## Maintenance Practices
- Use SEARCH/REPLACE blocks for surgical updates
- Maintain line number accuracy through changes
- Regular cross-references between docs and code
- Version control integration (git hashes in changelogs)
