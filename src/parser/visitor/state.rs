use crate::parser::graph::CodeGraph;
use crate::parser::nodes::{NodeId, ParameterNode};
use crate::parser::types::VisibilityKind;
use crate::parser::types::{GenericParamNode, TypeId};
use crate::parser::types::{TypeKind, TypeNode};
use crate::parser::visitor::utils::attributes::ParsedAttribute;
use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use syn::Type;
use syn::{FnArg, Visibility};

use super::processor::{StateManagement, TypeOperations};
use super::utils::{attributes, docs, generics};
use super::TypeProcessor;

pub struct VisitorState {
    pub code_graph: CodeGraph,
    pub next_node_id: NodeId,
    pub next_type_id: TypeId,
    pub type_map: HashMap<String, TypeId>,
}

// In src/parser/visitor/state.rs
impl StateManagement for VisitorState {
    // visibility qualifiers not permitted here AI!
    fn next_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }
    // visibility qualifiers not permitted here AI!
    fn next_type_id(&mut self) -> TypeId {
        let id = self.next_type_id;
        self.next_type_id += 1;
        id
    }
}

impl crate::parser::visitor::processor::DocOperations for VisitorState {
    fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String> {
        docs::extract_docstring(attrs)
    }
}

impl processor::AttributeOperations for VisitorState {
    fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
        attributes::extract_attributes(attrs)
    }
}

impl processor::GenericsOperations for VisitorState {
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
        generics::process_generics(self, generics)
    }

    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        generics::process_lifetime_bound(bound)
    }
}

impl TypeOperations for VisitorState {
    fn get_or_create_type(&mut self, ty: &Type) -> TypeId {
        let type_str = ty.to_token_stream().to_string();
        if let Some(&id) = self.type_map.get(&type_str) {
            return id;
        }

        let (type_kind, related_types) = self.process_type(ty);
        let id = self.next_type_id();
        self.type_map.insert(type_str, id);

        self.code_graph.type_graph.push(TypeNode {
            id,
            kind: type_kind,
            related_types,
        });

        id
    }
}

impl generics::GenericsProcessor for VisitorState {
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
        generics::process_generics(self, generics)
    }
}

impl docs::DocProcessor for VisitorState {
    fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String> {
        docs::extract_docstring(attrs)
    }
}

impl attributes::AttributeProcessor for VisitorState {
    fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
        attributes::extract_attributes(attrs)
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
}
