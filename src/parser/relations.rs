use crate::parser::graph_ids::GraphNodeId;
use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

// ANCHOR: Relation
// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RelationBatch {
    pub relations: Vec<Relation>,
    pub estimated_size: usize,
}

pub struct Relation {
    pub source: RelationSource,
    pub target: RelationTarget,
    pub graph_source: GraphNodeId,
    pub graph_target: GraphNodeId,
    pub kind: RelationKind,
}

impl Relation {
    pub fn new(
        source: impl Into<RelationSource> + Clone + Copy,
        target: impl Into<RelationTarget> + Clone + Copy,
        kind: RelationKind,
    ) -> Self {
        let source_val = source.into();
        let target_val = target.into();
        Self {
            source: source_val,
            target: target_val,
            graph_source: source_val.into(),
            graph_target: target_val.into(),
            kind,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl From<RelationSource> for GraphNodeId {
    fn from(source: RelationSource) -> Self {
        match source {
            RelationSource::Node(id) => GraphNodeId::from(id),
            RelationSource::Trait(id) => GraphNodeId::from(id),
        }
    }
}

impl From<RelationTarget> for GraphNodeId {
    fn from(target: RelationTarget) -> Self {
        match target {
            RelationTarget::Type(id) => GraphNodeId::from(id),
            RelationTarget::Trait(id) => GraphNodeId::from(id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    ImplementsTrait(TraitId), // TraitId type parameter added
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
