use crate::parser::graph_ids::GraphNodeId;
use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationBatch {
    pub relations: Vec<Relation>,
    pub estimated_size: usize,
}

// ANCHOR: Relation
// Represents a relation between nodes
// So we currently have RelationSource and RelationTarget, but these seem redundant. The only place
// it seems like they are ever used are as fields for Relation anyway, so why wouldn't we just make
// them GraphNodeId types instead? That way we can have a unified node type for everything. Is
// there a good reason not to do that? AI?
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub source: RelationSource,
    pub target: RelationTarget,
    pub graph_source: GraphNodeId,
    pub graph_target: GraphNodeId,
    pub kind: RelationKind,
}
//ANCHOR_END: Relation

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
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

impl From<NodeId> for RelationTarget {
    fn from(id: NodeId) -> Self {
        RelationTarget::Node(id)
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
