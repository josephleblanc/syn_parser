use crate::parser::nodes::NodeId;

use quote::ToTokens;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File as FsFile;
use std::io::Write;
use std::path::Path;
use syn::parse::Parser;
use syn::ItemMod;
use syn::{
    visit::{self, Visit},
    AngleBracketedGenericArguments, FnArg, GenericArgument, Generics, ItemEnum, ItemFn, ItemImpl,
    ItemStruct, ItemTrait, Pat, PatIdent, PatType, PathArguments, ReturnType, Type, TypeParam,
    TypePath, TypeReference, Visibility,
};

// ANCHOR: Relation
// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub source: NodeId,
    pub target: NodeId,
    pub kind: RelationKind,
}

// ANCHOR: Uses
// Different kinds of relations
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationKind {
    FunctionParameter,
    FunctionReturn,
    StructField,
    EnumVariant,
    ImplementsFor,
    ImplementsTrait,
    Inherits,
    References,
    Contains,
    Uses,
    ValueType,
    MacroUse,
    // MacroExpansion,
    // This is outside the scope of this project right now, but if it were to be implemented, it
    // would probably go here.
}
//ANCHOR_END: Uses
//ANCHOR_END: Relation
