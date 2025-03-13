use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

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
pub struct GraphNodeId {
    pub type_prefix: NodeType,
    pub unique_id: usize,
}

impl From<NodeId> for GraphNodeId {
    fn from(id: NodeId) -> Self {
        Self {
            type_prefix: NodeType::Node,
            unique_id: id.0,
        }
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
