# Goals

The goal of this project is to parse rust source files into data structures
that can be used to form a heterogeneous graph database. This database is
intended to be used by a Retrieval Augmented Generation (RAG) pipeline for
coding-related tasks such as code generation, code explain, code refactoring,
and documentation generation.

This project aims to achieve the goal of creating a RAG database for code by
parsing rust source files into structs that
can be used as-is or saved using `serde` to a `ron` file.

For construction of a heterogeneous graph, we identify `Item`s with `syn` that
will form nodes in the graph, and use static analysis to identify edge relations.
<!--Additionally we use [MIRAI](https://github.com/endorlabs/MIRAI) abstract-->
<!--interpreter to generate a call graph. More info on MIRAI's call graph-->
<!--[here](https://github.com/endorlabs/MIRAI/blob/main/documentation/CallGraph.md).-->

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
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
// Different kinds of types
#[derive(Debug, Serialize, Deserialize, Clone)]
    // ...
{{#include ../../../src/parser.rs:()}}
```

</details>
<details>
  <summary><code>ForeignMod(ItemForeignMod)</code> not implemented</summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Impl(ItemImpl)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>Item(Impl)</code> used for relations, needs more work.</summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:Item(Impl)}}
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

- [ ] `Mod(ItemMod)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>Mod(ItemMod)</code> not implemented, should be relation.</summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:ItemMod}}
```

</details>

- [ ] `Static(ItemStatic)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Struct(ItemStruct)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Trait(ItemTrait)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `TraitAlias(ItemTraitAlias)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Type(ItemType)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Union(ItemUnion)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Use(ItemUse)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

- [ ] `Verbatim(TokenStream)`
  - [ ] Started?
  - [ ] Finished?

<details>
  <summary><code>()</code> inside <code></code> </summary>

```rust,no_run,noplayground
{{#include ../../../src/parser.rs:()}}
```

</details>

2. No `Type` enum variants for now

As defined in `syn` documentation for [`Type`](https://docs.rs/syn/latest/syn/enum.Item.html).

These will just help with traversing the tree and go through temporary internal
representations in my code. This could be a stretch goal down the road.

- [ ] `Array(TypeArray)`
- [ ] `BareFn(TypeBareFn)`
- [ ] `Group(TypeGroup)`
- [ ] `ImplTrait(TypeImplTrait)`
- [ ] `Infer(TypeInfer)`
- [ ] `Macro(TypeMacro)`
- [ ] `Never(TypeNever)`
- [ ] `Paren(TypeParen)`
- [ ] `Path(TypePath)`
- [ ] `Ptr(TypePtr)`
- [ ] `Reference(TypeReference)`
- [ ] `Slice(TypeSlice)`
- [ ] `TraitObject(TypeTraitObject)`
- [ ] `Tuple(TypeTuple)`
- [ ] `Verbatim(TokenStream)`
