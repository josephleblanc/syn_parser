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

impl GraphNodeId {
    /// Convert to IndraDB compatible UUID v5 (name-based)
    pub fn to_uuid(&self) -> uuid::Uuid {
        let namespace = match self.type_prefix {
            NodeType::Node => uuid::Uuid::from_bytes([0x8A; 16]), // Placeholder namespace UUIDs
            NodeType::Trait => uuid::Uuid::from_bytes([0x8B; 16]),
            NodeType::Type => uuid::Uuid::from_bytes([0x8C; 16]),
            NodeType::Module => uuid::Uuid::from_bytes([0x8D; 16]),
            NodeType::Function => uuid::Uuid::from_bytes([0x8E; 16]),
            NodeType::Impl => uuid::Uuid::from_bytes([0x8F; 16]),
        };
        
        uuid::Uuid::new_v5(
            &namespace,
            &self.unique_id.to_le_bytes()
        )
    }
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
