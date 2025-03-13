use crate::parser::nodes::NodeId;

use serde::{Deserialize, Serialize};

/// Unique identifier for type references
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TypeId(pub usize);

impl From<TypeId> for NodeId {
    fn from(id: TypeId) -> Self {
        NodeId(id.0)
    }
}

impl From<TypeId> for usize {
    fn from(id: TypeId) -> Self {
        id.0
    }
}

impl TypeId {
    pub fn as_node_id(&self) -> Option<NodeId> {
        Some(NodeId::from(self.0))
    }
    
    pub fn as_usize(&self) -> usize { self.0 }
}

// Temporary alias for gradual migration
pub type LegacyTypeId = usize;

impl std::ops::AddAssign<usize> for TypeId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

// ANCHOR: TypeNode
// Represents a type reference with full metadata
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
