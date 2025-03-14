Documentation should:

## Core Principles
1. **As-Is Accuracy** - Document only verified code existence
2. **Line-Anchored References** - Use exact line numbers (e.g., `nodes.rs:67-72`)
3. **Anti-Speculation** - No "planned" or "would need" statements
4. **Dependency Proofs** - Show actual `use` statements from code
5. **Negative Space** - Document absence (empty files, unused modules)

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
1. **Change Validation**:
   - SEARCH blocks must match exact line numbers
   - Verify against `git blame` for recent changes
2. **Speculation Checks**:
   - Scan for "planned", "would", "future" markers
   - Replace with code references or remove
3. **Empty State**:
   - Document blank files/modules as empty
   - No placeholder documentation
4. **Diagram Grounding**:
   - Mermaid elements must match `mod.rs` structure
   - Remove non-existent relationships
