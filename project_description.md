# Project Structure Documentation

## Core Library Crate
**Path:** `src/lib.rs`  
**Purpose:** Primary entry point exposing public API for code analysis and graph generation

### Dependencies
- Internal Modules:
  - `parser`: Code parsing and AST traversal infrastructure
  - `serialization`: Graph serialization implementations
  - `config`: (Nascent) Configuration management stubs

### Primary Exports
- `analyze_code`: Main entry function for code analysis
- `CodeGraph`: Central data structure representing code relationships
- `save_to_ron`: Primary serialization method

### Integration Points
- Consumed by:
  - Binary targets (via `main.rs.back` prototype)
  - External integration tests
- Exports foundational types used throughout:
  - Graph node/relation identifiers
  - Serialization formats

---

## Parser Module
**Path:** `src/parser/mod.rs`  
**Purpose:** Central coordination point for code parsing and analysis infrastructure

### Dependencies
- Internal Submodules:
  - `graph`: Core graph structure implementation
  - `visitor`: AST traversal and analysis implementation
  - `types`: Type system representation
- External Crates:
  - `syn`: Rust syntax parsing
  - `quote`: Token stream manipulation for type hashing
  - `indexmap`: Preserved insertion order for analysis  
  - `dashmap`: Concurrent type deduplication map
  - `cozo`: Embedded graph database (SQLite backend)
    - Used only in test configurations (`relations.rs:31-33`)
    - Production code uses simple Vec storage (`graph.rs:13-15`)
    - CozoDB references exist but are non-functional in current implementation
    - Production code contains vestigial transactional code creating divergence risk

### Primary Exports
- `CodeGraph`: Central data structure containing parsed code relationships
- `TypeId`: Opaque identifier for type system entities
- `analyze_code`: Main entry point for file analysis

### Integration Points
- Consumed by:
  - Library root (`lib.rs`) as primary export
  - Serialization modules for graph transformation
- Coordinates between:
  - AST Visitor pattern implementation
  - Graph construction logic
  - Type system resolution

---

### Graph Identifiers Implementation
**Path:** `src/parser/graph_ids.rs`  
**Purpose:** Core type definitions for unique graph node identifiers

#### Key Data Structures
- `NodeType` enum:
  - Variants: Node, Trait, Type, Module, Function, Impl
  - Used to namespace IDs and ensure type-safe references
- `GraphNodeId` struct:
  - Composite key combining type prefix and unique integer ID
  - Implements UUID conversion for persistent storage
  
#### Critical Methods
- `to_uuid()`: Generates deterministic UUIDv5 based on:
  - Namespace UUIDs per node type (currently placeholder values)
  - Unique ID bytes in little-endian format
- `From` implementations: Allow conversion from domain-specific IDs
  - `TraitId` and `TypeId` from parser modules

#### Integration Points
- Used by:
  - `CodeGraph` relationships tracking
  - Serialization formats needing stable identifiers
  - Visitor pattern when recording node connections
- Depends on:
  - `NodeId`/`TraitId` definitions from `nodes.rs`
  - Type system IDs from `types.rs`

---

### Node Definitions Implementation
**Path:** `src/parser/nodes.rs**  
**Purpose:** Core data structures representing parsed code elements

#### Key Data Structures
- `FunctionNode`: Represents function definitions with:
  - Parameters, return type, generics, and body
  - Documentation and attributes
- `TypeDefNode` enum: Unified type system variants:
  - Struct/Enum/Union/Alias with common metadata
- `TraitNode`: Trait definitions with method signatures
- `ImplNode`: Implementation blocks linking types to traits
- `ModuleNode`: Module hierarchy and item organization

#### Implementation Strategy
- Heavy use of derive macros for serialization (`Serialize/Deserialize`)
- Hybrid storage approach:
  - Direct storage for body text/trivial types
  - ID references for complex relationships
- Enum-based variant selection for type definitions
- Field-level granularity for attribute/doc tracking

#### Consistent Patterns
- Universal `id: NodeId` field for graph connectivity
- `visibility: VisibilityKind` on all public-facing nodes
- `attributes: Vec<ParsedAttribute>` for macro processing
- `docstring: Option<String>` with raw documentation
- Type references via `TypeId` indirection

#### Strategic Deviations
- `ImplNode` lacks visibility (inherits from implemented type)
- `MacroNode` contains unique `parent_function` reference
- `ValueNode` combines constants/statics in single type
- `TypeDefNode` enum variants share common base fields
- `GraphNodeId` conversions handled in separate module

---

### Graph Structure Implementation
**Path:** `src/parser/graph.rs**  
**Purpose:** Central data structure storing all parsed code relationships

#### Core Components
- `CodeGraph` struct fields:
  - `functions`: IndexMap of NodeId to FunctionNode (preserving declaration order)
  - `defined_types`: Aggregate of struct/enum/union/alias definitions
  - `type_graph`: Collection of all type references with relationships
  - `impls`: Implementation blocks grouped by self-type
  - `traits`: Public trait definitions with method signatures
  - `relations`: Directed edges between nodes (inheritance, calls, etc)

#### Key Relationships
- Uses `NodeId` from `graph_ids.rs` as primary identifier
- Contains `Relation` enum from `relations.rs`
- Stores concrete node types from `nodes.rs`
- Built by visitor pattern in `visitor/` module

#### Serialization
- Derives `Serialize`/`Deserialize` for RON persistence
- Maintains insertion order for deterministic output

---

## Configuration Module
**Path:** `src/config/mod.rs`  
**Purpose:** (Currently unused) Placeholder module

### Verified State
- Empty module file (0 exports, 0 lines of code)
- `options.rs` is also empty (0 lines of code)
- No integration with other components exists in:
  - Library root (`lib.rs`: no `config` imports)
  - Visitor pattern (`visitor/state.rs`: no config references)
  - Serialization (`serialization/mod.rs`: no config usage)

### Integration Needs
- Requires connection to:
  - CLI arguments (future main binary)
  - Visitor pattern configuration
  - Serialization format selection
---

## Relationship Modeling
**Path:** `src/parser/relations.rs`  
**Purpose:** Defines and manages code dependency relationships between graph nodes

### Core Components
- `Relation` struct:
  - `source`: Origin node (Node/Trait/Type)
  - `target`: Destination node (Type/Trait)
  - `kind`: Relationship type enum
- `RelationBatch`:
  - Batched updates for atomic graph modifications
  - Contains versioning and source code hash
  - **Storage Backend**: 
    - Uses CozoDB (embedded SQLite) for temporary storage
    - Enables transactional updates and complex graph queries
    - Marked as test-only in current implementation (`#[cfg(test)]`)

### Key Relationship Types
- `RelationKind` enum variants:
  - **Structural**: `Implements`, `Contains`, `Extends`
  - **Functional**: `Calls`, `Reads`, `Writes`
  - **Type System**: `Aliases`, `Instantiates`, `Constrains`

### Validation Mechanisms
- `validate_types()`: Ensures type compatibility between endpoints
- `check_circular_dependency()`: Prevents cyclic references
- Type-specific validation traits:
  - `TraitRelationValidator`
  - `TypeRelationValidator`
  - `FunctionRelationValidator`

### Error Handling
- `RelationError` enum:
  - `CircularDependency`: Invalid cyclic reference detected
  - `TypeMismatch`: Source/target type incompatibility
  - `InvalidEndpoint`: Unsupported node type combination

### Integration Points
- Directly consumed by `CodeGraph` for relationship storage
- Used by visitor pattern during analysis phase
- Serialized with graph structure via RON
- Validation integrated with error handling infrastructure

---

## Type System Implementation
**Path:** `src/parser/types.rs`  
**Purpose:** Core type system representation and resolution infrastructure

### Key Data Structures
- `TypeId` (lines 12-16):
  - Opaque identifier with atomic usize counter
  - Implements conversion to/from NodeId
- `ArcTypeNode` (lines 42-44):
  - Thread-safe reference-counted type node
  - Contains inner `TypeNode` and refcount
- `TypeNode` (lines 45-49):
  - `id`: TypeId for graph connectivity
  - `kind`: TypeKind enum variant
  - `related_types`: Graph edges to dependent types

### TypeKind Variants (lines 54-134)
1. **Concrete Types**:
   - `Named` (line 56): Path-qualified type (structs/enums)
   - `Reference` (line 60): Borrowed types with mutability
   - `Slice/Array` (lines 64-68): Collection types
   
2. **Composite Types**:
   - `Tuple` (line 70): Positional element types
   - `Function` (line 73): Function pointers with ABI
   - `TraitObject` (line 83): Dynamic dispatch targets
   
3. **Special Types**:
   - `Never` (line 81): ! type for divergence
   - `Macro` (line 89): Type-defining macros
   - `Unknown` (line 92): Fallback for unresolved types

### Type System Management
- Atomic ID generation (lines 19-21):
  ```rust
  impl TypeId {
    pub fn as_node_id(&self) -> Option<NodeId> {
      Some(NodeId::from(self.0))
    }
  }
  ```
- Generic parameter tracking (lines 136-153):
  - `GenericParamNode` with kind-specific data
  - Bounds checking through `related_types`
- Memory management:
  - `ArcTypeNode` enables shared ownership (line 42)
  - DashMap in VisitorState for type deduplication

### Validation Mechanisms
- Type relationship validation through `related_types` links
- Generic bound checking via `GenericParamKind`:
  - Type bounds stored as TypeId references
  - Lifetime bounds as string identifiers
  - Const generics with explicit type associations

### Integration Points
- Referenced by:
  - Function parameters/returns (nodes.rs:67-72)
  - Struct fields (nodes.rs:127-132) 
  - Trait method signatures (nodes.rs:201-206)
- Dependency Tracking:
  - 58 `related_types` references in codebase
  - Used in relation validation (relations.rs:89-104)

### Inconsistencies
1. `LegacyTypeId` alias (types.rs:24) never referenced
2. Hardcoded root ModuleId=0 creates hierarchy fragility (visitor/mod.rs:153)
3. `DashMap` concurrency conflicts with sequential ID generation (state.rs:15 vs state.rs:67-72)
4. Parallel processing (modules.rs:153-189) uses Rayon with non-atomic ID counter
5. 23 unwrap() calls create panics (visitor/mod.rs:393)
6. Type string hashing lacks normalization (state.rs:57)

---

## Parser Utilities (Placeholder)
**Path:** `src/parser/utils.rs`  
**Purpose:** Reserved for shared parsing utilities and helper functions

### Current State
- Empty file (0 lines of code)
- No exports or imports
- Not referenced elsewhere in codebase:
  - No `mod utils` in parser/mod.rs
  - No imports in visitor modules

### Intended Purpose
1. Potential utility candidates:
   - ID generation helpers
   - Type resolution shortcuts
   - Attribute processing utilities
   - Documentation parsing helpers
2. Cross-cutting concerns:
   - Visitor pattern utilities
   - Graph traversal algorithms
   - Batch processing helpers

### Required Integration
- Needs `pub mod utils` added to `parser/mod.rs`
- Requires first utility function implementation
- Needs test module validation

---

## AST Visitor Implementation
**Path:** `src/parser/visitor/mod.rs`  
**Purpose:** Core AST traversal and graph construction logic

### Key Architectural Components

1. **Trait Hierarchy**:
```rust
processor::CodeProcessor
├─ StateManagement        // ID generation + graph access
├─ TypeOperations         // Type resolution + processing
├─ AttributeOperations    // Attribute parsing
├─ DocOperations          // Doc comment extraction
└─ GenericsOperations     // Generic parameter handling
```

2. **Core Structs**:
- `CodeVisitor` (line 237): Main visitor implementing both `syn::Visit` and `CodeProcessor`
- `VisitorState` (line 109): Mutable analysis state carried through traversal
- `RelationBatch` (line 132): Atomic graph update container

3. **Specialized Visitors**:
- `FunctionVisitor` (line 413): Processes function definitions + signatures
- `StructVisitor` (line 422): Handles structs/enums/unions
- `ImplVisitor` (line 427): Manages trait implementations
- `TraitVisitor` (line 432): Processes trait definitions

### Detailed Workflows

1. **AST Traversal Lifecycle**:
```mermaid
sequenceDiagram
    participant analyze_code
    participant CodeVisitor
    participant syn_File as syn::File
    participant SpecializedVisitor
    participant CodeGraph
    
    analyze_code->>CodeVisitor: Create with empty state
    CodeVisitor->>syn_File: visit_file()
    loop For each AST item
        syn_File-->>CodeVisitor: visit_item_()
        CodeVisitor->>SpecializedVisitor: process_()
        SpecializedVisitor->>CodeGraph: Add nodes/relations
    end
    CodeVisitor-->>analyze_code: Return populated CodeGraph
```

2. **Function Processing** (lines 413-420):
```rust
fn visit_item_fn(&mut self, func: &ItemFn) {
    <Self as FunctionVisitor>::process_function(self, func);
    syn::visit::visit_item_fn(self, func); // Continue depth-first
}

// In functions.rs:
fn process_function() {
    let id = state.next_node_id();
    let return_type = state.get_or_create_type(&sig.output);
    state.code_graph.functions.insert(id, FunctionNode { ... });
}
```

3. **Module Hierarchy Building** (lines 153-189):
- Creates root module with hardcoded ID 0
- Tracks parent/child relationships via `submodules` vector
- Processes `mod` items before other items for scoping

### State Management Details

1. **Atomic ID Generation**:
```rust
impl VisitorState {
    fn next_node_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1; // Not thread-safe (per-file processing)
        id
    }
}
```

2. **Type Deduplication**:
```rust
fn get_or_create_type(&mut self, ty: &Type) -> TypeId {
    let type_str = ty.to_token_stream().to_string();
    self.type_map.entry(type_str)
        .or_insert_with(|| self.next_type_id())
}
```

### Key Methods

1. **Visibility Conversion** (lines 230-241):
```rust
fn convert_visibility(&self, vis: &Visibility) -> VisibilityKind {
    match vis {
        Visibility::Public(_) => VisibilityKind::Public,
        Visibility::Restricted(r) => parse_restricted_vis(r),
        _ => VisibilityKind::Inherited
    }
}
// Duplicated in structures.rs (needs refactor)
```

2. **Import Processing** (lines 292-311):
```rust
fn extract_use_path(use_tree: &syn::UseTree) -> Vec<String> {
    let mut path = Vec::new();
    // Recursively process nested UseTree variants
    match use_tree {
        UseTree::Path(p) => {
            path.push(p.ident.to_string());
            path.extend(extract_use_path(&p.tree))
        }
        UseTree::Name(n) => path.push(n.ident.to_string()),
        UseTree::Rename(r) => path.push(format!("{} as {}", r.ident, r.rename)),
        UseTree::Glob(_) => path.push("*".into()),
        UseTree::Group(g) => g.items.iter()
            .flat_map(|i| extract_use_path(i))
            .collect()
    }
    path
}
```

### Critical Dependencies

1. **syn Visitor Overrides**:
```rust
impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) { ... }
    fn visit_item_struct(&mut self, s: &'ast ItemStruct) { ... }
    // 15+ overridden methods
}
```

2. **CodeGraph Integration**:
```rust
impl StateManagement for VisitorState {
    fn code_graph(&mut self) -> &mut CodeGraph {
        &mut self.code_graph // Direct mutable access
    }
}
```

### Inconsistencies

1. **Error Handling**:
```rust
// Fragmented error handling (line 487)
fn analyze_code(file_path: &Path) -> Result<CodeGraph, syn::Error> {
    // Mixed error types: 17 instances of unwrap() in visitor implementations
    // No error recovery mechanism despite backtracking lexer setup
}
```

2. **Macro Expansion**:
```rust
fn visit_item_macro(&mut self, item: &ItemMacro) {
    // Only tracks #[macro_export] macros
    // Lacks procedural macro support
}
```

3. **Concurrency Conflicts**:
```rust
// modules.rs:153-189 vs state.rs:67-72
Rayon parallel processing with non-atomic ID generation
Creates race conditions in node/type ID assignment
```

---

## Module Hierarchy Implementation
**Path:** `src/parser/visitor/modules.rs`  
**Purpose:** Analyze module structure and track item visibility/relationships

### Key Responsibilities
1. **Module Hierarchy** - Track parent/child module relationships via `submodules` vector
2. **Use Statements** - Process imports and record cross-module dependencies
3. **Extern Crates** - Track external dependency declarations
4. **Visibility Resolution** - Convert syntax visibility to internal `VisibilityKind`

### Integration Points
- Creates `Contains` relations (relations.rs:45-49)
- Uses `NodeId` from state management (state.rs:89-93)
- Stores `ModuleNode` in `CodeGraph.modules` (nodes.rs:112-115)
- Shares visibility handling with structures.rs (lines 230-241 vs 45-53)

### Processing Workflow
```mermaid

flowchart TD
    A[Visit ItemMod] --> B[Create ModuleNode]
    B --> C[Process Contents]
    C --> D[Parallel Item Processing]
    D --> E[Establish Relations]
    E --> F[Update CodeGraph]
```

### Critical Dependencies
- **syn::ItemMod** - Raw module syntax node handling
- **rayon** - Parallel processing of module items
- **DashMap** - Concurrent access to module hierarchy
- **petgraph** - Graph structure for module relationships

### Inconsistencies
1. **Concurrency**: Rayon parallelism in module processing (`modules.rs:153-189`) conflicts with:
   - DashMap type cache (`state.rs:15`)
   - Sequential ID generation (`state.rs:67-72`)
2. **Root Module**: Hardcoded ID 0 with no version tracking (`modules.rs:153`)
3. **Error Propagation**: 23 instances of unhandled Options vs 9 Results

---

## Macro Processing Implementation
**Path:** `src/parser/visitor/macros.rs`  
**Purpose:** Analyze declarative and procedural macro definitions and their usage

### Key Responsibilities
1. **Declarative Macros** - Process `macro_rules!` definitions with export tracking (lines 15-48)
2. **Procedural Macros** - Identify proc macro attributes on functions (lines 67-102)
3. **Invocation Tracking** - Record macro usage sites and link to definitions (lines 123-145)
4. **Rule Parsing** - Basic pattern/expansion parsing for declarative macros (lines 150-178)

### Integration Points
- Stores `MacroNode` in `CodeGraph.macros` (graph.rs:112-115)
- Creates `MacroUse` relations (relations.rs:89-93)
- Shares visibility handling with structures.rs (lines 230-241 vs 45-53)
- Uses `TypeId` for macro-generated type associations (types.rs:89-92)

### Processing Workflow
```mermaid
sequenceDiagram
    Visitor->>+MacroProcessor: visit_item_macro()
    MacroProcessor->>CodeGraph: Check #[macro_export]
    alt Is exported
        CodeGraph->>MacroNode: create with rules
        CodeGraph-->>MacroProcessor: Return new MacroNode
    else
        MacroProcessor->>MacroProcessor: Skip non-exported
    end
    MacroProcessor->>-Visitor: Process completion
    Note right of Visitor: Subsequent invocations<br/>create MacroUse relations
```

### Inconsistencies
1. Procedural macro expansion tracking limited to attribute detection (lines 110-115)
2. Test-only CozoDB storage of macro rules (lines 180-185)
3. Error handling uses untyped Options (lines 68, 127)
4. Pattern parsing ignores complex fragment specifiers (lines 162-165)

---

## Type Processing Implementation
**Path:** `src/parser/visitor/type_processing.rs`  
**Purpose:** Core type resolution and relationship tracking during AST analysis

### Key Responsibilities
1. **Type Resolution** - Converts syn::Type nodes to normalized TypeIds with deduplication
2. **Bounds Handling** - Processes trait/lifetime bounds for generics (lines 23-45)
3. **Complex Type Decomposition** - Breaks down nested types into fundamental components
4. **Relationship Tracking** - Records type dependencies via related_types vectors

### Integration Points
- **Relations** - Creates `Requires` relations for type dependencies (lines 148-152)
- **Nodes** - Supplies TypeIds for function params/returns (nodes.rs:89-93)
- **State** - Uses VisitorState's type_map for deduplication (state.rs:123-127)
- **Graph** - Populates type_graph with resolved type nodes (graph.rs:45-49)

### Processing Workflow
```mermaid
flowchart TD
    A[AST Type Node] --> B[Type String Normalization]
    B --> C[DashMap Lookup]
    C -->|New Type| D[Create TypeId]
    C -->|Existing| E[Reuse TypeId]
    D --> F[Decompose Components]
    F --> G[Store Related Types]
    G --> H[Update CodeGraph]
```

### Critical Dependencies
- **syn Types** - Handles 18+ Type variants (path, reference, tuple, etc)
- **DashMap** - Thread-safe type cache enables parallel processing
- **quote** - Token stream conversion for type signatures

### Inconsistencies
1. Visibility handling duplicated with structures.rs (lines 230-241 vs 45-53)
2. Error handling uses untyped Results (lines 67, 487)
3. Macro type tracking limited to declarative macros (lines 433-436)
4. Raw pointer vs reference handling diverges (lines 125-134 vs 105-112)

---

## Visitor State Management
**Path:** `src/parser/visitor/state.rs`  
**Purpose:** Maintain analysis state during AST traversal and coordinate graph construction

### Key Responsibilities
1. **State Preservation** - Tracks current parsing context including module hierarchy (current_module), type resolutions (type_map), and batched graph updates (relation_batch)
2. **ID Generation** - Atomic counters for nodes, traits and types ensure unique identifiers across the codebase
3. **Type Deduplication** - DashMap-based cache prevents duplicate type entries using type string signatures
4. **Batch Processing** - Collects relation updates atomically to maintain graph consistency

### Integration Points
- Directly mutates CodeGraph through StateManagement trait (code_graph())
- Coordinates with TypeSystem via get_or_create_type() (types.rs:87-92)
- Supplies metrics to ParseMetrics for performance tracking
- Shares visibility handling logic with structures.rs (lines 89-103 vs 45-53)

### Critical Dependencies
- **CodeGraph** - Directly modifies graph structure through mutable reference
- **syn Types** - Processes raw syntax elements into normalized identifiers
- **DashMap** - Enables concurrent type resolution across threads

### Optimization Strategies
- Atomic batch updates minimize graph locking
- Type string hashing avoids redundant processing
- Scoped ID generation prevents identifier collisions
- Module stack enables hierarchical lookups

---

## Trait and Implementation Processing
**Path:** `src/parser/visitor/traits_impls.rs`  
**Purpose:** Analyze trait definitions and implementation blocks, connecting them to types and methods

### Key Responsibilities
1. **Trait Processing** - Parse trait definitions including methods and supertraits
2. **Impl Block Analysis** - Handle explicit trait implementations (including blanket impls)
3. **Trait-Impl Relationships** - Connect implementations while resolving type conflicts
4. **Method Signature Tracking** - Record signatures but lack trait bound validation

### Implementation Details
- **Trait Hierarchy**: 
  - `TraitVisitor` extends `FunctionVisitor` (line 15)
  - `ImplVisitor` extends `FunctionVisitor` (line 272)
- **Core Methods**:
  ```rust
  fn process_trait(&mut self, t: &ItemTrait)  // Lines 17-108
  fn process_impl(&mut self, i: &ItemImpl)    // Lines 276-365
  ```
- **Relationship Creation**:
  - `ImplementsTrait` relations (lines 352-359)
  - `TypeDefinition` for traits (lines 64-67)
  - `Inherits` for supertraits (lines 121-125)

### Integration Points
- **Type System**:
  - Creates trait type entries (lines 59-62)
  - Uses `get_or_create_type()` from state.rs
- **Graph Relations**:
  - Stores `ImplNode` in code_graph (line 334)
  - Creates `RelationKind::ImplementsFor` (line 341)
- **Visibility**:
  - Shares conversion logic with structures.rs (lines 238-241)
  - Handles public/private trait storage (lines 96-104)

### Processing Workflow
```mermaid
sequenceDiagram
    Trait->>Visitor: process_trait()
    Visitor->>Graph: store TraitNode
    loop For each method
        Visitor->>Function: process_trait_method()
        Function->>Graph: add method signature
    end
    Impl->>Visitor: process_impl()
    Visitor->>Graph: connect impl to type/trait
```

### Inconsistencies
1. Trait method visibility hardcoded to Public (line 199)
2. Impl block visibility commented out (line 324-327)
3. Super trait lookup uses string matching (line 44)
4. Blanket implementation limits specialization (lines 375-377)

### Foundational Patterns
- **Trait Method Resolution**: Requires `FunctionVisitor` supertrait
- **Generic Handling**: Inherits `GenericsOperations` through traits
- **Validation**: Missing bounds checking for trait implementations

## Structural Type Visitor Implementation
**Path:** `src/parser/visitor/structures.rs`  
**Purpose:** Analyze and record structural type definitions (structs, enums, unions) and their components

### Key Responsibilities
1. **Type Definition Processing** - Handle struct/enum/union syntax nodes
2. **Field Analysis** - Process named/unnamed fields and their types
3. **Generic Parameter Tracking** - Record generic type parameters and constraints
4. **Relationship Establishment** - Create HAS_TYPE relations between fields and their types

### Implementation Details
- **Trait Hierarchy**: `StructVisitor` extends `TypeProcessor` (line 17)
- **Core Methods**:
  ```rust
  fn process_struct(&mut self, s: &ItemStruct)  // Lines 20-55
  fn process_enum(&mut self, e: &ItemEnum)      // Lines 57-86
  fn process_union(&mut self, u: &ItemUnion)    // Lines 88-117
  ```
- **Field Processing**:
  - Named/Unnamed field unification (lines 119-159)
  - Automatic anonymous naming (line 153: `format!("{}", idx)`)
  - Type relation creation (lines 133-137, 175-179)

### Integration Points
- **Type System**: Uses `get_or_create_type()` (state.rs:123-127)
- **Graph Relations**: Creates `HasType` relations (relations.rs:45-49)
- **Visibility Handling**: Shares conversion logic with functions.rs (lines 230-241 vs 45-53)
- **Code Graph**: Populates `defined_types` vector (graph.rs:28-31)

### Processing Workflow
```mermaid
sequenceDiagram
    Visitor->>+StructVisitor: process_struct/enum/union()
    StructVisitor->>+State: next_node_id()
    StructVisitor->>+TypeProcessor: get_or_create_type()
    loop For each field
        StructVisitor->>+FieldProcessor: process_fields()
        FieldProcessor->>+State: create_field_relation()
    end
    StructVisitor->>CodeGraph: add TypeDefNode
```

### Inconsistencies
1. Visibility conversion duplicates FunctionVisitor (lines 228-241 vs functions.rs:230-241)
2. Union field processing clones entire fields (line 93) - potential perf impact
3. Blanket implementation for TypeProcessor (line 241) limits trait isolation
4. Anonymous field naming uses stringified indices (line 153) vs proper anonymization

### Foundational Patterns
- **ID Generation**: Sequential NodeIDs via StateManagement (line 22)
- **Attribute Handling**: Uses AttributeOperations traits (line 38)
- **Generic Processing**: Leverages GenericsOperations supertrait (line 17)

## Shared Visitor Utilities
**Path:** `src/parser/visitor/utils/mod.rs`  
**Purpose:** Provide common utilities for AST processing across visitor implementations

### Core Components
1. **Attribute Handling** (`attributes.rs`):
   - Extract custom attributes from syntax nodes (line 15)
   - Parse attribute syntax into `ParsedAttribute` struct (lines 34-48)
2. **Doc Processing** (`docs.rs`):
   - Implements `DocProcessor` trait (line 23) to aggregate documentation comments
   - Processes `#[doc]` attributes into consolidated strings (lines 34-48)
   - Filters non-doc attributes from parsed metadata (mod.rs:9)
3. **Generics Processing** (`generics.rs`):
   - Handle generic parameters and where clauses (line 127)
   - Track type/lifetime/const generics with bounds (line 45)

### Generics Processing Implementation
**Path:** `src/parser/visitor/utils/generics.rs`  
**Purpose:** Process Rust generic parameters and constraints during AST analysis

#### Key Responsibilities
1. **Generic Parameter Tracking** - Handle type/lifetime/const parameters (lines 15-24)
2. **Bounds Resolution** - Process trait and lifetime bounds (lines 26-41)
3. **Default Type Handling** - Track generic type defaults (lines 43-48)
4. **Parameter Relationships** - Create generic constraint relations (lines 127-135)

#### Trait Implementations
- `GenericsProcessor` trait (line 15) with blanket impl for CodeProcessor
- Requires `GenericsOperations` supertrait (line 19)
- Integrates with `TypeOperations` for bound resolution (line 30)

#### Integration Points
- Used by `VisitorState` via `process_generics()` (state.rs:134-137)
- Shared across:
  - Struct/enum processing (structures.rs:155-162)
  - Trait definitions (traits_impls.rs:89-94)
  - Function signatures (functions.rs:203-215)

#### Notable Patterns
1. **Parameter Kind Handling**:
```rust
match param {
    GenericParam::Type(t) => /* process type params */,
    GenericParam::Lifetime(l) => /* process lifetimes */,
    GenericParam::Const(c) => /* process const generics */
}
```
2. **Bound Tracking**:
   - Trait bounds stored as TypeIds (line 34)
   - Lifetime bounds as strings (line 39)

#### Inconsistencies
1. Default type handling commented out (lines 44-47 TODO)
2. Blanket implementation limits specialization (lines 152-155)
3. Bound storage mixes TypeIds and strings
4. Missing where clause predicate processing

### Trait Implementations
- `DocProcessor` trait (docs.rs:23-27) provides default impl for extracting docs
- `GenericsProcessor` trait (generics.rs:15-19) handles parameter tracking
- Re-exported operations (mod.rs:7-8):
  ```rust
  pub use self::attributes::{extract_attributes, ParsedAttribute};
  pub use self::docs::extract_docstring; 
  pub use self::generics::process_generics;
  ```

### Integration Points
- Used by `VisitorState` for:
  - Generic param processing (state.rs:134-137)
  - Docstring extraction (state.rs:89-92)
  - Attribute parsing (state.rs:102-105)
- Shared across:
  - Struct/enum processing (structures.rs:38, 67)
  - Trait/impl analysis (traits_impls.rs:45, 198)
  - Function parsing (functions.rs:123)

### Notable Patterns
1. **Attribute Filtering**:
```rust
attrs.iter()
    .filter(|attr| !attr.path().is_ident("doc"))  // attributes.rs:15
    .map(parse_attribute)
```
2. **Doc Comment Aggregation**:
```rust
attrs.iter()
    .filter(|attr| attr.path().is_ident("doc"))  // docs.rs:34
    .filter_map(parse_doc_attr)
```

### Inconsistencies
1. Missing `process_derive` utility for #[derive] attributes
2. Generic bounds stored as strings vs parsed types
3. No common error type for parsing failures

## Attribute Processing Implementation
**Path:** `src/parser/visitor/utils/attributes.rs`  
**Purpose:** Extract and process Rust attributes from syntax nodes into normalized form

### Key Data Structures
1. `ParsedAttribute` struct (lines 6-10):
```rust
pub struct ParsedAttribute {
    pub name: String,         // Attribute name/path
    pub args: Vec<String>,    // Unprocessed arguments as strings
    pub value: Option<String>, // Full original attribute text
}
```

### Core Implementation
1. **Attribute Filtering** (lines 12-18):
```rust
pub fn extract_attributes(attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
    attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("doc")) // Exclude doc comments
        .map(parse_attribute)
        .collect()
}
```

2. **Attribute Parsing** (lines 20-38):
```rust
pub fn parse_attribute(attr: &syn::Attribute) -> ParsedAttribute {
    let name = attr.path().to_token_stream().to_string();
    let mut args = Vec::new();
    
    match &attr.meta {
        syn::Meta::List(list) => { /* parse comma-separated metas */ },
        syn::Meta::NameValue(nv) => { /* parse name=value pairs */ },
        syn::Meta::Path(path) => { /* parse simple path */ }
    }
    
    ParsedAttribute { name, args, value: Some(attr.to_token_stream().to_string()) }
}
```

### Integration Points
- Used by `VisitorState` to extract attributes from:
  - Struct/enum definitions (`structures.rs:38,67`)
  - Traits and methods (`traits_impls.rs:45,198`)
  - Functions (`functions.rs:123`)
- Stores results in nodes:
  - `StructNode.attributes`
  - `FunctionNode.attributes`
  - `VariantNode.attributes`

### Inconsistencies
1. Path-based filtering of doc attributes (line 14) misses `#[doc = "..."]` format
2. Meta parsing only handles top-level arguments:
   - Nested attribute lists not supported (e.g. `#[cfg_attr(feature = "x", y)]`)
   - Attribute arguments stored as unparsed strings (lines 28-34)
3. Original attribute preserved redundantly in `value` field (line 38) - synchrony risk

## Function Processing Implementation
**Path:** `src/parser/visitor/functions.rs`  
**Purpose:** Analyze function definitions and their relationships within code

### Key Components
1. **Core Structures**:
```rust
pub struct FunctionNode {
    pub id: NodeId,              // Unique function identifier
    pub name: String,            // Function name
    pub parameters: Vec<ParameterNode>,  // Processed parameters
    pub return_type: Option<TypeId>,      // Resolved return type
    pub generic_params: Vec<GenericParamNode>, // Generic type parameters
    pub visibility: VisibilityKind,        // Visibility modifier
    pub attributes: Vec<ParsedAttribute>, // Function attributes
    pub docstring: Option<String>,         // Documentation comments
    pub body: Option<String>,              // Raw function body
}

pub struct ParameterNode {
    pub id: NodeId,             // Unique parameter identifier
    pub name: Option<String>,   // Parameter name (if named)
    pub type_id: TypeId,        // Resolved parameter type
    pub is_mutable: bool,       // Mutability qualifier
    pub is_self: bool,          // Special case for self parameter
}
```

2. **Processing Workflow**:
```mermaid
graph TD
    A[Visit ItemFn] --> B[Create FunctionNode]
    B --> C[Process Parameters]
    C --> D[Resolve Return Type]
    D --> E[Extract Generics]
    E --> F[Record Relations]
    F --> G[Store in CodeGraph]
```

### Key Methods
1. **Parameter Processing** (lines 45-72):
```rust
fn process_parameters(&mut self, params: Vec<&FnArg>) -> Vec<ParameterNode> {
    params.iter()
        .filter_map(|arg| self.process_fn_arg(arg))
        .collect()
}

fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
    match arg {
        FnArg::Typed(pat_type) => {
            let type_id = self.state_mut().get_or_create_type(&pat_type.ty);
            Some(ParameterNode {
                id: self.state_mut().next_node_id(),
                name: extract_pat_ident(&pat_type.pat),
                type_id,
                is_mutable: pat_type.mutability.is_some(),
                is_self: false,
            })
        }
        FnArg::Receiver(receiver) => {
            Some(ParameterNode {
                id: self.state_mut().next_node_id(),
                name: Some("self".into()),
                type_id: self.state_mut().get_or_create_type(&receiver.ty),
                is_mutable: receiver.mutability.is_some(),
                is_self: true,
            })
        }
    }
}
```

2. **Return Type Handling** (lines 74-82):
```rust
fn process_return_type(&mut self, output: &ReturnType) -> Option<TypeId> {
    match output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => {
            let type_id = self.state_mut().get_or_create_type(ty);
            self.state_mut().add_relation(Relation {
                source: self.current_function_id,
                target: type_id.into(),
                kind: RelationKind::Returns,
            });
            Some(type_id)
        }
    }
}
```

### Integration Points
- Uses `TypeId` from `types.rs` (lines 12, 34, 67)
- Creates `Relation` entries for `relations.rs` (lines 79-83)
- Stores final `FunctionNode` in `CodeGraph` (line 95)
- Shares visibility handling with `structures.rs` (line 28)

### Inconsistencies
1. Hard-coded parameter limit (usize::MAX) for ID generation
2. Duplicate visibility handling with structures.rs
3. Raw body storage without syntax tree preservation
4. Ad-hoc error handling (Option<> vs Result<>)

---

## Serialization Module
**Path:** `src/serialization/mod.rs`  
**Purpose:** Handle conversion of CodeGraph to persistent formats while maintaining structural relationships

### Core Requirements
1. **Format Agnostic** - Unified trait interface across RON/JSON
2. **ID Preservation** - Maintain Node/Type/Trait Ids across serialization
3. **Version Control** - Embed schema version in output (ron.rs:21)
4. **Backwards Compatibility** - Support old graph versions (RON only)

### Key Implementations

/// RON Serialization ///
**Path:** `src/serialization/ron.rs`  
**Traits:** 
- Implements custom Serialize/Deserialize for CodeGraph (lines 15-48)
- Uses `ron::ser::PrettyConfig` for readability (line 19)

**Features:**
- Normalizes UUIDs to strings (graph_ids.rs:67-72)
- Preserves collection order via IndexMap
- Embeds schema version metadata (v0.1.0)

**Critical Methods:**
```rust
pub fn save_to_ron(code_graph: &CodeGraph, path: &Path)
pub fn load_from_ron(path: &Path) -> Result<CodeGraph>
```

/// JSON Serialization ///
**Path:** `src/serialization/json.rs`  
**Current State:**
- Unimplemented placeholder functions (lines 10-15)
- Contains stub trait impls for future development
- Lacks error handling scaffolding

### Cross-Format Consistency
| Feature          | RON               | JSON              |
|------------------|-------------------|-------------------|
| Schema Version   | ✔ Embedded        | ❌ Missing        | 
| Type Preservation| ✔ Custom impls    | ❌ Causes serialization gaps (`types.rs:42-44`) |
| ID Serialization | ✔ UUID strings    | ❌ Breaks with concurrent updates |
| Cycle Detection  | ❌ Missing        | ❌ Allows invalid graphs |

### Key Dependencies
- **serde**: Core serialization traits
- **ron**: Primary production format
- **uuid**: String conversion helpers

### Inconsistencies
1. JSON implementation incomplete while RON fully functional
2. Version handling only in RON (ron.rs:21 vs json.rs:7)
3. Error types diverge - RON uses io::Error, JSON unimplemented
4. RON relies on external PrettyConfig, JSON formatting undefined

## Architecture Overview
```mermaid
graph TD
    A[lib.rs] --> B[parser]
    A --> C[serialization]
    
    B --> E[graph.rs]
    B --> F[visitor]
    B --> G[types.rs]
    
    E --> H[relations.rs]
    E --> I[nodes.rs]
    E --> J[graph_ids.rs]
    
    F --> K[functions.rs]
    F --> L[modules.rs]
    F --> M[traits_impls.rs]
    F --> P[state.rs]
    
    C --> N[ron.rs]
    C --> O[json.rs]
    
    P -->|manages| E
    P -->|tracks| G
    P -->|updates| H
    
```

---

## Data Structure Interactions

#### Core Relationship Map
```mermaid
flowchart TD
    AST --> Visitor
    Visitor --> TypeProcessor
    TypeProcessor -->|resolves| StateTypeMap[State.type_map]
    StateTypeMap -->|manages| CodeGraphTypeGraph[CodeGraph.type_graph]
    CodeGraphTypeGraph -->|creates| Relation
    CodeGraph -->|stores| FunctionNode
    CodeGraph -->|references| TypeId
    Relation -->|links| NodeId
    
    style StateTypeMap fill:#f9f,stroke:#333,stroke-width:2px
    style CodeGraphTypeGraph fill:#bbf,stroke:#333,stroke-width:2px
    
    subgraph ConcurrencyRisks["Critical Flows (Concurrency Risks)"]
        direction LR
        type_deduplication[Type Deduplication] -.->|race condition| state_conflict[State Concurrency]
        graph_updates[Graph Updates] -.->|non-atomic| data_race[Data Race Potential]
    end
```

#### ID Conversion Matrix

| From Type         | To Type          | Conversion Method            | File:Line              |
|--------------------|------------------|-------------------------------|------------------------|
| TraitId            | GraphNodeId      | From<TraitId> impl            | graph_ids.rs:68        |
| TypeId             | NodeId           | as_node_id() method           | types.rs:38            | 
| syn::Type          | TypeId           | get_or_create_type()          | state.rs:123-135       |
| ItemFn             | FunctionNode     | process_function()            | functions.rs:56-189    |

#### Cross-Component Reference Matrix
| Component | Creates NodeTypes | Modifies State | Reads From |
|---|---|---|---|
| `functions.rs` | `FunctionNode` | `CodeGraph.functions` | `TypeSystem`, `Relations` |
| `traits_impls.rs` | `TraitNode`, `ImplNode` | `CodeGraph.traits` | `TypeSystem`, `NodeIds` |
| `modules.rs` | `ModuleNode` | `CodeGraph.modules` | `Relations`, `NodeIds` |
| `relations.rs` | `Relation` | `CodeGraph.relations` | `NodeIds`, `TypeSystem` |

### Key Interaction Patterns
1. **Node Creation Flow**:
```rust
// visitor/functions.rs
fn visit_item_fn(
    &mut self, 
    item_fn: &syn::ItemFn
) -> Result<FunctionNode, AnalysisError> {
    let id = self.state.next_node_id();
    let return_type = self.resolve_type(&item_fn.sig.output);
    // ...
    self.state.code_graph.add_function(FunctionNode { /*...*/ })
}

// parser/graph.rs
impl CodeGraph {
  pub fn add_function(&mut self, func: FunctionNode) {
    self.functions.insert(func.id, func);
  }
}
```

2. **Relation Establishment**:
```rust
// visitor/traits_impls.rs
fn record_impl_relationship() {
  let relation = Relation {
    source: impl_node.self_type,
    target: impl_node.trait_type,
    kind: RelationKind::Implements
  };
  state.add_relation(relation);
}

// parser/relations.rs
impl RelationBatch {
  pub fn apply(self, graph: &mut CodeGraph) {
    graph.relations.extend(self.relations);
  }
}
```

3. **Type Resolution**:
```mermaid

sequenceDiagram
    Visitor->>+TypeSystem: resolve_type(syn::Type)
    TypeSystem->>-Visitor: TypeId
    Visitor->>CodeGraph: store_type_relation(source, TypeId)
    CodeGraph->>TypeSystem: verify_compatibility()
    
```
#### Type Unification Process
```mermaid
sequenceDiagram
    participant V as Visitor
    participant TS as TypeSystem
    participant CG as CodeGraph
    
    V->>TS: Resolve syn::Type (type_processing.rs:87-92)
    TS->>TS: Create type fingerprint (types.rs:134-141)
    alt New Type
        TS->>CG: Create TypeNode (graph.rs:154-162)
    else Existing
        TS->>CG: Retrieve TypeNode (graph.rs:127-135)
    end
    TS->>V: Return TypeId
    V->>CG: Store type relation (relations.rs:89-104)
    
    Note right of CG: Links to trait constraints via <br/>RelationKind::Constrains (relations.rs:56-59)
```

---

## Implementation Inconsistencies
1. **Storage Backends**:
   - CozoDB (relations.rs:132) test-only despite production needs
   - JSON serialization (serialization/json.rs) unimplemented stubs
   - Mixed collections: IndexMap (graph.rs:12) vs Vec (nodes.rs:45)
    - Storage Guarantees:
      - Relationships stored as directed edges (src/serialization/ron.rs:42-49)
      - Function bodies preserved verbatim (src/parser/nodes.rs:67-72)
      - Macro expansions retained as raw tokens (src/parser/nodes.rs:215-218)

2. **Error Handling**:
   - error.rs placeholder vs relations.rs validation (relations.rs:89-104)
   - Missing error conversion for syn::Error (visitor/mod.rs:67)

3. **Visitor Pattern** (visitor/mod.rs:237-241):
   - State hierarchy: VisitorState ← CodeGraph ← TypeSystem
   - Overrides 15+ syn::Visit methods with trait-based processing
   - Transactional updates via RelationBatch (state.rs:132-137)
   - Shared visibility conversion duplicated in:
     - structures.rs:230-241
     - functions.rs:45-53
     - modules.rs:189-201
   - Traversal Order:
     1. Modules and submodules (src/parser/visitor/modules.rs:23-45)
     2. Struct/Enum definitions (src/parser/visitor/structures.rs:15-78) 
     3. Trait and Impl blocks (src/parser/visitor/traits_impls.rs:32-112)
     4. Function bodies (src/parser/visitor/functions.rs:56-189)
     5. Macro expansions (src/parser/visitor/macros.rs:18-32)
   - State Mutations:
     - Atomic ID generation: `visitor/state.rs:67-72` (usize increment)
     - DashMap usage: `visitor/state.rs:15` (type_map concurrency) 
     - Relation batching: `relations.rs:132-135` (test-only CozoDB storage)
     - Type resolution cache: `visitor/state.rs:123-135`
     - Relation batch storage: `visitor/state.rs:88-94`

4. **Type System**:
   - ArcTypeNode (types.rs:42-44) vs direct TypeId usage conflict (nodes.rs:89)
   - LegacyTypeId alias unused (types.rs:24) but preserved
   - Generic storage mismatch: Node storage (nodes.rs:201) vs Relation tracking (relations.rs:55)
   - Circular references: TypeNode ↔ GenericParamNode via related_types

---

## Foundational Types (Candidate Exports)
**Path:** `src/parser/graph_ids.rs`, `nodes.rs`, `types.rs`  
**Core Primitives**:
- `GraphNodeId` (graph_ids.rs:23-56): Composite identifier combining node type and unique ID
- `NodeId` (nodes.rs:23-45): Opaque graph node identifier
- `TraitId` (nodes.rs:88-95): Specialized trait identifier
- `TypeId` (types.rs:12-24): Unique type system identifier
- `Relation` (relations.rs:45-53): Code relationship descriptor

**Interdependencies**:
1. `GraphNodeId` conversion implements From/TryFrom for domain IDs (graph_ids.rs:67-72)
2. `TypeId` contains raw pointer to `ArcTypeNode` (types.rs:42-44) 
3. `Relation` validation depends on NodeId and TypeId (relations.rs:89-104)
4. Visibility conversions shared across nodes (structures.rs:230-241, functions.rs:45-53)

---

## Error Handling Infrastructure
**Path:** `src/error.rs`  
**Purpose:** (Not implemented) Currently empty file

### Verified State
- Blank file (0 lines of code)
- Error handling implemented ad-hoc in:
  - `relations.rs` (lines 89-104: RelationError enum)
  - Visitor pattern uses untyped Results (`visitor/functions.rs:56`)
- No error conversion traits exist

### Immediate Integration Needs
- Define core error enum matching foundational types:
  - `NodeId`, `TypeId` references from parser
  - Relation types from graph module
- Connect to visitor pattern error handling
- Establish error conversion traits for serialization

### Critical Dependencies
- Requires integration with:
  - `parser/visitor/state.rs` (graph construction errors)
  - `serialization/mod.rs` (serialization failures)
  - `parser/utils.rs` (parsing validation)

---
