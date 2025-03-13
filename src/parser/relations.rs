use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

// ANCHOR: Relation
// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub source: RelationSource,
    pub target: RelationTarget,
    pub kind: RelationKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationSource {
    Node(NodeId),
    Trait(TraitId),
}

impl From<NodeId> for RelationSource {
    fn from(id: NodeId) -> Self {
        RelationSource::Node(id)
    }
}

impl From<TraitId> for RelationSource {
    fn from(id: TraitId) -> Self {
        RelationSource::Trait(id)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RelationTarget {
    Type(TypeId),
    Trait(TraitId),
}

impl From<TypeId> for RelationTarget {
    fn from(id: TypeId) -> Self {
        RelationTarget::Type(id)
    }
}

impl From<TraitId> for RelationTarget {
    fn from(id: TraitId) -> Self {
        RelationTarget::Trait(id)
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
    ImplementsTrait(TraitId),  // Now properly typed
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
