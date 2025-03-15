use std::sync::{atomic::AtomicUsize, Arc};

use crate::parser::nodes::NodeId;

use serde::{Deserialize, Serialize};

use super::graph_ids::{self, GraphIdentifier, GraphNodeId};

// AI: As it says below TypeId was supposed to be a unique identifier for types, I imagine in an
// effort to distinguish the path of different types,
// e.g. `Vec<&str>` vs `std::vec::Vec<&str>`
// However, at some point it seems like it got mixed up with the unique identifier for NodeId,
// which has an entirely different purpse.
/// Unique identifier for type references
#[deprecated = "Use GraphNodeId with NodeType::Type instead"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TypeId(pub u64);

impl GraphIdentifier for TypeId {
    fn from_graph_id(id: super::graph_ids::GraphNodeId) -> Option<Self> {
        (id.type_prefix == graph_ids::NodeType::Type).then_some(TypeId(id.unique_id))
    }
    fn as_graph_id(&self) -> graph_ids::GraphNodeId {
        GraphNodeId {
            type_prefix: graph_ids::NodeType::Type,
            unique_id: self.0,
        }
    }
}

// AI:
// The following should probably never have been implemented.
impl From<TypeId> for NodeId {
    fn from(id: TypeId) -> Self {
        NodeId(id.0)
    }
}

// AI: This either
impl TypeId {
    pub fn as_node_id(&self) -> Option<NodeId> {
        Some(NodeId::from(self.0))
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

// AI: I have no idea why this is here. I feel like I'm walking through a forest of spaghetti
// Temporary alias for gradual migration
pub type LegacyTypeId = usize;

impl std::ops::AddAssign<usize> for TypeId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

// ANCHOR: TypeNode
// Represents a type reference with full metadata

#[cfg(feature = "concurrency_migration")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ArcTypeNode {
    inner: Arc<TypeNode>,
    ref_count: AtomicUsize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeNode {
    pub id: TypeId,
    pub kind: TypeKind,
    // Reference to related types (e.g., generic arguments)
    pub related_types: Vec<TypeId>,
}
//ANCHOR_END: TypeNode

// ANCHOR: TypeKind_defn
// Different kinds of types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeKind {
    //ANCHOR_END: TypeKind_defn
    Named {
        path: Vec<String>, // Full path segments
        is_fully_qualified: bool,
    },
    Reference {
        lifetime: Option<String>,
        is_mutable: bool,
        // Type being referenced is in related_types[0]
    },
    Slice {
        // Element type is in related_types[0]
    },
    Array {
        // Element type is in related_types[0]
        size: Option<String>,
    },
    Tuple {
        // Element types are in related_types
    },
    // ANCHOR: ExternCrate
    Function {
        // Parameter types are in related_types (except last one)
        // Return type is in related_types[last]
        is_unsafe: bool,
        is_extern: bool,
        abi: Option<String>,
    },
    //ANCHOR_END: ExternCrate
    Never,
    Inferred,
    RawPointer {
        is_mutable: bool,
        // Pointee type is in related_types[0]
    },
    // ANCHOR: TraitObject
    TraitObject {
        // Trait bounds are in related_types
        dyn_token: bool,
    },
    //ANCHOR_END: TraitObject
    // ANCHOR: ImplTrait
    ImplTrait {
        // Trait bounds are in related_types
    },
    //ANCHOR_END: ImplTrait
    Paren {
        // Inner type is in related_types[0]
    },
    // ANCHOR: ItemMacro
    Macro {
        name: String,
        tokens: String,
    },
    //ANCHOR_END: ItemMacro
    Unknown {
        type_str: String,
    },
}

// Represents a generic parameter
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericParamNode {
    pub id: NodeId,
    pub kind: GenericParamKind,
}

// ANCHOR: generic_param_kind
// Different kinds of generic parameters
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GenericParamKind {
    Type {
        name: String,
        bounds: Vec<TypeId>,
        default: Option<TypeId>,
    },
    Lifetime {
        name: String,
        bounds: Vec<String>,
    },
    Const {
        name: String,
        type_id: TypeId,
    },
}
//ANCHOR_END: generic_param_kind

// Different kinds of visibility
/// Visibility modifier for code items
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum VisibilityKind {
    Public,
    Crate,
    Restricted(Vec<String>), // Path components of restricted visibility
    Inherited,               // Default visibility
}
