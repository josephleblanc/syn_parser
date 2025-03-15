use crate::parser::{
    nodes::{FunctionNode, ImplNode, MacroNode, ModuleNode, TraitNode, TypeDefNode, ValueNode},
    relations::Relation,
    types::TypeNode,
};

use serde::{Deserialize, Serialize};

// Main structure representing the entire code graph
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGraph {
    // Functions defined in the code
    pub functions: Vec<FunctionNode>,
    // Types (structs, enums) defined in the code
    pub defined_types: Vec<TypeDefNode>,
    // All observed types, including nested and generic types
    pub type_graph: Vec<TypeNode>,
    // Implementation blocks
    // // Changing how visibility is handled. It should not be tracked by the impl nodes but
    // in the methods and types the impl nodes implement.
    // pub public_impls: Vec<ImplNode>,
    // pub private_impls: Vec<ImplNode>,
    pub impls: Vec<ImplNode>,
    // Public traits defined in the code
    pub traits: Vec<TraitNode>,
    // Private traits defined in the code
    pub private_traits: Vec<TraitNode>,
    // Relations between nodes
    pub relations: Vec<Relation>,
    // Modules defined in the code
    pub modules: Vec<ModuleNode>,
    // Constants and static variables
    pub values: Vec<ValueNode>,
    // Macros defined in the code
    pub macros: Vec<MacroNode>,
}

// This is a second version of the CodeGraph we can use as we start migrating to a concurrent
// model.
#[cfg(features = "concurrency_migration")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGraph {
    // Functions defined in the code - use DashMap for concurrent access
    pub functions: dashmap::DashMap<NodeId, FunctionNode>,
    // Types (structs, enums) defined in the code
    pub defined_types: dashmap::DashSet<TypeDefNode>,
    // All observed types, including nested and generic types
    pub type_graph: dashmap::DashSet<TypeNode>,
    // Implementation blocks
    pub impls: dashmap::DashSet<ImplNode>,
    // Public traits defined in the code
    pub traits: dashmap::DashSet<TraitNode>,
    // Private traits defined in the code
    pub private_traits: dashmap::DashSet<TraitNode>,
    // Relations between nodes
    pub relations: dashmap::DashSet<Relation>,
    // Modules defined in the code
    pub modules: dashmap::DashSet<ModuleNode>,
    // Constants and static variables
    pub values: dashmap::DashSet<ValueNode>,
    // Macros defined in the code
    pub macros: dashmap::DashSet<MacroNode>,
}
