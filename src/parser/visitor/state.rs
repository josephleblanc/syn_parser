use crate::parser::graph::CodeGraph;
use crate::parser::nodes::{NodeId, ParameterNode};
use crate::parser::types::TypeId;
use crate::parser::types::VisibilityKind;
use crate::parser::types::{TypeKind, TypeNode};
use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use syn::{FnArg, Visibility};

use super::processor::StateManagement;
use super::TypeProcessor;

pub struct VisitorState {
    pub code_graph: CodeGraph,
    pub next_node_id: NodeId,
    pub next_type_id: TypeId,
    pub type_map: HashMap<String, TypeId>,
}

// In src/parser/visitor/state.rs
impl StateManagement for VisitorState {
    pub fn next_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }
    pub fn next_type_id(&mut self) -> TypeId {
        let id = self.next_type_id;
        self.next_type_id += 1;
        id
    }
}

impl VisitorState {
    pub fn new() -> Self {
        Self {
            code_graph: CodeGraph {
                functions: Vec::new(),
                defined_types: Vec::new(),
                type_graph: Vec::new(),
                impls: Vec::new(),
                traits: Vec::new(),
                private_traits: Vec::new(),
                modules: Vec::new(),
                values: Vec::new(),
                macros: Vec::new(),
                relations: Vec::new(),
            },
            next_node_id: 0,
            next_type_id: 0,
            type_map: HashMap::new(),
        }
    }
}
