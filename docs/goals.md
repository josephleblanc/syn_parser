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

1. All `Item` enum variants defined in `syn` documentation for `Item`:

- [ ] `Fn(ItemFn)`
  - [x] Started?
  - [ ] Finished?
- [ ] `Const(ItemConst)`
  - [x] Started?
  - [ ] Finished?
  - Currently
- [ ] `Enum(ItemEnum)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `ExternCrate(ItemExternCrate)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `ForeignMod(ItemForeignMod)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Impl(ItemImpl)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Macro(ItemMacro)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Mod(ItemMod)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Static(ItemStatic)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Struct(ItemStruct)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Trait(ItemTrait)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `TraitAlias(ItemTraitAlias)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Type(ItemType)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Union(ItemUnion)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Use(ItemUse)`
  - [ ] Started?
  - [ ] Finished?
- [ ] `Verbatim(TokenStream)`
  - [ ] Started?
  - [ ] Finished?

2. All `Type` enum variants defined in `syn` documentation for [`Type`](https://docs.rs/syn/latest/syn/enum.Item.html):

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
