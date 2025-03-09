# Goals

This document contains the overview of desired functionality and a list
tracking feature implementation.

The goal of this project is to parse rust source files into data structures
that can be used to form a heterogeneous graph database. This database is
intended to be used by a Retrieval Augmented Generation (RAG) pipeline for
coding-related tasks such as code generation, code explain, code refactoring,
and documentation generation.

This project aims to achieve the goal of creating a RAG database for code by
parsing rust source files into structs that
can be used as-is or saved using `serde` to a `ron` or `json` file.

For construction of a heterogeneous graph, we identify `Item`s with [`syn`] that
will form nodes in the graph, and use static analysis to identify edge relations.

## Nodes

There should be a node for each kind of rust "Item" (see [rust reference](https://doc.rust-lang.org/reference/items.html)).

| Rust Items | In-Project | `syn` [Item] | Progress | Next Step |
| --------------------- | ----------------- | ----------------------- | ----------------------- | --------------- |
| [Module] | todo | [ItemMod] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [ExternCrate] | [Function::is_extern] | [ItemExternCrate] | ☑ Start <br> ☐ Finish | [Edges Todo] |
| [UseDeclaration] | [RealtionKind::Uses] | [ItemUse] | ☑ Start <br> ☐ Finish | [Edges Todo] |
| [Function] | todo | [ItemFn] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [TypeAlias] | todo | [ItemType] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Struct] | todo | [ItemStruct] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Enumeration] | todo | [ItemEnum] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Union] | todo | [ItemUnion] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [ConstantItem] | todo | [ItemConst] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [StaticItem] | todo | [ItemStatic] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Trait] | todo | [ItemTrait] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Implementation] | todo | [ItemImpl] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [ExternBlock] | todo | [ItemForeignMod] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Macro] | todo | [ItemMacro] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [TraitAlias] | todo | [ItemTraitAlias] | ☐ Start <br> ☐ Finish | [Nodes Todo] |
| [Linking modifiers: verbatim] | todo | [Verbatim] | ☐ Start <br> ☐ Finish | [Nodes Todo] |

[Module]:https://doc.rust-lang.org/reference/items/modules.html
[ExternCrate]:https://doc.rust-lang.org/stable/reference/items/extern-crates.html
[UseDeclaration]:https://doc.rust-lang.org/stable/reference/items/use-declarations.html
[Function]:https://doc.rust-lang.org/reference/items/functions.html
[TypeAlias]:https://doc.rust-lang.org/reference/items/type-aliases.html
[Struct]:https://doc.rust-lang.org/reference/items/structs.html
[Enumeration]:https://doc.rust-lang.org/reference/items/enumerations.html
[Union]:https://doc.rust-lang.org/reference/items/unions.html
[ConstantItem]:https://doc.rust-lang.org/reference/items/constant-items.html
[StaticItem]:https://doc.rust-lang.org/reference/items/static-items.html
[Trait]:https://doc.rust-lang.org/reference/items/traits.html
[Implementation]:https://doc.rust-lang.org/reference/items/implementations.html
[ExternBlock]:https://doc.rust-lang.org/reference/items/external-blocks.html
[Macro]:https://doc.rust-lang.org/reference/macros-by-example.html
[TraitAlias]: https://doc.rust-lang.org/reference/items/type-aliases.html#trait-aliases
[Linking modifiers: verbatim]:https://doc.rust-lang.org/reference/items/external-blocks.html?highlight=Verbatim#linking-modifiers-verbatim

[`syn`]:https://docs.rs/syn/latest/syn/index.html
[Item]:https://docs.rs/syn/latest/syn/enum.Item.html
[ItemMod]:https://docs.rs/syn/latest/syn/struct.ItemMod.html
[ItemExternCrate]:https://docs.rs/syn/latest/syn/struct.ItemExternCrate.html
[ItemUse]:https://docs.rs/syn/latest/syn/struct.ItemUse.html
[ItemFn]:https://docs.rs/syn/latest/syn/struct.ItemFn.html
[ItemType]:https://docs.rs/syn/latest/syn/struct.ItemType.html
[ItemStruct]:https://docs.rs/syn/latest/syn/struct.ItemStruct.html
[ItemEnum]:https://docs.rs/syn/latest/syn/struct.ItemEnum.html
[ItemUnion]:https://docs.rs/syn/latest/syn/struct.ItemUnion.html
[ItemConst]:https://docs.rs/syn/latest/syn/struct.ItemConst.html
[ItemStatic]:https://docs.rs/syn/latest/syn/struct.ItemStatic.html
[ItemTrait]:https://docs.rs/syn/latest/syn/struct.ItemTrait.html
[ItemImpl]:https://docs.rs/syn/latest/syn/struct.ItemImpl.html
[ItemForeignMod]:https://docs.rs/syn/latest/syn/struct.ItemForeignMod.html
[ItemMacro]:https://docs.rs/syn/latest/syn/struct.ItemMacro.html
[ItemTraitAlias]:https://docs.rs/syn/latest/syn/struct.ItemTraitAlias.html
[Verbatim]:https://docs.rs/syn/latest/syn/enum.Item.html#variant.Verbatim

[Nodes Todo]:./todo_nodes.md
[Edges Todo]:./todo_edges.md
[RealtionKind::Uses]:./nodes.md
[Function::is_extern]:./nodes.md
