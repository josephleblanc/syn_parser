# Nodes

Nodes are represented by `struct` in this project, and will be reprsented as
entries in the graph database. Nodes are connected by edges, which are
represented as fields in their `struct`. Here we detail the desired
implementation for node identification and data extraction.

## Types

The `syn` crate makes it possible to parse rust source files for many of the
data structures used by the `rustc` compiler. However, not all of these data
structures will be useful for our graph database. The `syn` crate primarily
parses rust source files into `Type`, `Item`, and `Expr` structures (see `syn`
[docs](https://docs.rs/syn/latest/syn/index.html) for more info).

We process the following `syn` data structures to form nodes and later perform
static analysis.

### 1. All `Item`s in `syn`

<details>
  <summary><code>Fn(ItemFn)</code></summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:ItemFn}}
```

</details>

<details>
  <summary><code>Const(ItemConst)</code> inside <code>GenericParamKind</code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:generic_param_kind}}
```

</details>

<details>
  <summary><code>Enum(ItemEnum)</code> as <code>FieldNode</code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:field_node}}
```

</details>

<details>
  <summary><code>ExternCrate(ItemExternCrate)</code> inside <code>TypeKind::Function::is_extern</code> </summary>

```rust,no_run,noplayground
// Different kinds of types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeKind {
    // ...
{{#include ../../../src/parser.rs:ExternCrate}}
```

</details>
<details>
  <summary><code>ForeignMod(ItemForeignMod)</code> not implemented</summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

<details>
  <summary><code>Item(Impl)</code> used for relations, needs more work.</summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:ItemNode}}
```

</details>
<details>
  <summary><code>Macro(ItemMacro)</code> inside <code>TypeKind</code> </summary>

```rust,no_run,noplayground
// Different kinds of types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeKind {
    // ...
{{#include ../../../src/parser.rs:ItemMacro}}
```

</details>

<details>
  <summary><code>Mod(ItemMod)</code> not implemented, should be relation.</summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:ItemMod}}
```

</details>
<details>
  <summary><code>Static(ItemStatic)</code> not implemented.</summary>

```rust,no_run,noplayground
```

</details>
<details>
  <summary><code>Struct(ItemStruct)</code></summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:StructNode}}
```

</details>

<details>
  <summary><code>Trait(ItemTrait)</code> as <code>TraitNode</code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:TraitNode}}
```

</details>

<details>
  <summary><code>TraitAlias(ItemTraitAlias)</code> not implemented. </summary>

</details>

<details>
  <summary><code>Type(ItemType)</code> as <code>TypeNode</code></summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:TypeNode}}
```

</details>

<details>
  <summary><code>Union(ItemUnion)</code> not implemented. </summary>

</details>

<details>
  <summary><code>Use(ItemUse)</code> inside <code>RelationKind</code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:Uses}}
```

</details>

<details>
  <summary><code>Verbatim(TokenStream)</code> not implemented. </summary>

</details>

### 2. Only `Type` enum variants for now

As defined in `syn` documentation for [`Type`](https://docs.rs/syn/latest/syn/enum.Type.html).

These will just help with traversing the tree and going through temporary
internal representations in my code. This could be a stretch goal down the
road.

These two types will be useful for extracting relations:

<details>
  <summary><code>ImplTrait(TypeImplTrait)</code> inside <code>TypeKind</code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:TypeKind_defn}}
    // ...
{{#include ../../../src/parser.rs:ImplTrait}}
    // ...
}
```

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:TypeNode}}
```

</details>

<details>
  <summary><code>TraitObject(TypeTraitObject)</code> inside <code>TypeNode</code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:TypeKind_defn}}
    // ...
{{#include ../../../src/parser.rs:TraitObject}}
    // ...
}
```

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:TypeNode}}
```

</details>

<details>
  <summary>Other <code>Type</code> enum variants, not implemented as relations</summary>

- `Array(TypeArray)`
- `BareFn(TypeBareFn)`
- `Group(TypeGroup)`
- `Infer(TypeInfer)`
- `Macro(TypeMacro)`
- `Never(TypeNever)`
- `Paren(TypeParen)`
- `Path(TypePath)`
- `Ptr(TypePtr)`
- `Reference(TypeReference)`
- `Slice(TypeSlice)`
- `Tuple(TypeTuple)`
- `Verbatim(TokenStream)`

</details>

Helpful example:

- Item 1
- <details>
    <summary>Item 2 with Details</summary>
    <p>Details for item 2. You can have multiple lines of content here.</p>
  </details>
- Item 3
