use crate::parser::nodes::NodeId;
use serde::{Deserialize, Serialize};

// ANCHOR: Relation
// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub source: NodeId,
    pub target: NodeId, // Temporary alias during transition
    pub kind: RelationKind,
}

impl Relation {
    /// Temporary conversion method for TypeId targets
    pub fn with_type_target(mut self, target: TypeId) -> Self {
        self.target = target.into();
        self
    }
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
    TypeDefinition,
    Uses,
    ValueType,
    MacroUse,
    MacroExpansion,
    MacroDefinition,
    MacroInvocation,
    GenericParameter,
    Returns,
    // TODO: Consider removing `HasType` later.
    // I don't think it's really useful but am wrestling with bugs rn.
    HasType,
}
//ANCHOR_END: Uses
//ANCHOR_END: Relation
