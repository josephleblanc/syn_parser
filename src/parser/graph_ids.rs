use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Node,
    Trait,
    Type,
    Module,
    Function,
    Impl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// causing an error here in relation.rs, which I've copied below:
// #[error("Invalid implementation relationship between {source} and {target}")]
// InvalidImplementation {
//     source: GraphNodeId,
//     target: GraphNodeId,
//     kind: RelationKind,
// },
// └╴  doesn't satisfy `graph_ids::GraphNodeId: AsDynError<'_>` or `graph_ids::GraphNodeId: StdError` rustc (E0599) [17, 1]
pub struct GraphNodeId {
    pub type_prefix: NodeType,
    pub unique_id: usize,
}

impl std::error::Error for GraphNodeId {}

impl fmt::Display for GraphNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}#{}", self.type_prefix, self.unique_id)
    }
}

impl From<TraitId> for GraphNodeId {
    fn from(id: TraitId) -> Self {
        Self {
            type_prefix: NodeType::Trait,
            unique_id: id.0,
        }
    }
}

impl From<TypeId> for GraphNodeId {
    fn from(id: TypeId) -> Self {
        Self {
            type_prefix: NodeType::Type,
            unique_id: id.0,
        }
    }
}
