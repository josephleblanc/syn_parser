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
    pub functions: IndexMap<NodeId, FunctionNode, ahash::RandomState>,
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
