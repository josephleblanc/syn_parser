use crate::parser::graph::CodeGraph;
use crate::parser::nodes::{NodeId, ParameterNode};
use crate::parser::types::TypeId;
use crate::parser::types::VisibilityKind;
use crate::parser::types::{TypeKind, TypeNode};
use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use syn::{FnArg, Visibility};

use super::TypeProcessor;

pub struct VisitorState {
    pub code_graph: CodeGraph,
    pub next_node_id: NodeId,
    pub next_type_id: TypeId,
    pub type_map: HashMap<String, TypeId>,
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

    pub fn convert_visibility(&self, vis: &Visibility) -> VisibilityKind {
        match vis {
            Visibility::Public(_) => VisibilityKind::Public,
            Visibility::Restricted(restricted) => {
                let path = restricted
                    .path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect();
                VisibilityKind::Restricted(path)
            }
            _ => VisibilityKind::Inherited,
        }
    }

    // Finish this function below AI!
    pub fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
        match arg {
            FnArg::Typed(pat_type) => {
                let param_id = self.next_node_id();
                let param_name = pat_type.pat.to_token_stream().to_string();
                let type_id = self.get_or_create_type(&pat_type.ty);

                Some(ParameterNode {
                    id: param_id,
                    name: Some(param_name),
                    type_id,
                    // placeholder
                    // TODO: handle cases for mutable/immutable parameters and is_self AI!
                    is_mutable: todo!(),
                    is_self: todo!(),
                    // How should we handle this?
                    // is_mutable:,
                    // How should we handle this?
                    // is_self: todo!(),
                })
            }
            _ => None,
        }
    }
}
