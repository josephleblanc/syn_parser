# Goals

The goal of this project is to parse rust source files into data structures
that can be used to form a heterogeneous graph database. This database is
intended to be used by a Retrieval Augmented Generation pipeline (RAG) for
coding-related tasks (code generation, code explain, code refactoring).

This project achieves this goal by parsing rust source files into structs that
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

1. All `Type` enum variants defined in `syn` documentation for [`Type`](https://docs.rs/syn/latest/syn/enum.Item.html):

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

2. All `Item` enum variants defined in `syn` documentation for `Item`:

- [ ] `Const(ItemConst)`
- [ ] `Enum(ItemEnum)`
- [ ] `ExternCrate(ItemExternCrate)`
- [x] `Fn(ItemFn)`
- [ ] `ForeignMod(ItemForeignMod)`
- [ ] `Impl(ItemImpl)`
- [ ] `Macro(ItemMacro)`
- [ ] `Mod(ItemMod)`
- [ ] `Static(ItemStatic)`
- [ ] `Struct(ItemStruct)`
- [ ] `Trait(ItemTrait)`
- [ ] `TraitAlias(ItemTraitAlias)`
- [ ] `Type(ItemType)`
- [ ] `Union(ItemUnion)`
- [ ] `Use(ItemUse)`
- [ ] `Verbatim(TokenStream)`
