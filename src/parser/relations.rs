use crate::parser::nodes::NodeId;
use serde::{Deserialize, Serialize};

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
    MacroExpansion,
    MacroDefinition,
    MacroInvocation,
    GenericParameter,
}
//ANCHOR_END: Uses
//ANCHOR_END: Relation
