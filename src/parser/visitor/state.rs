use crate::parser::graph::CodeGraph;
use crate::parser::nodes::FunctionNode;
use crate::parser::nodes::{NodeId, ParameterNode};
use crate::parser::relations::Relation;
use crate::parser::types::VisibilityKind;
use crate::parser::types::{GenericParamNode, TypeId};
use crate::parser::types::{TypeKind, TypeNode};
use crate::parser::visitor::utils::attributes::ParsedAttribute;
use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use syn::Type;
use syn::{FnArg, Visibility};

use super::processor;
use super::processor::{
    AttributeOperations, CodeProcessor, DocOperations, GenericsOperations, StateManagement,
    TypeOperations,
};
use super::utils::{attributes, docs, generics};

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
            next_node_id: NodeId(0),
            next_type_id: TypeId(0),
            type_map: HashMap::new(),
        }
    }

    // Process function arguments for both self and CodeVisitor usage
    pub fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
        match arg {
            FnArg::Typed(pat_type) => {
                let param_id = self.next_node_id();
                let (name, is_mutable) = match &*pat_type.pat {
                    syn::Pat::Ident(ident) => {
                        (Some(ident.ident.to_string()), ident.mutability.is_some())
                    }
                    _ => (None, false),
                };

                let type_id = self.state_mut().get_or_create_type(&pat_type.ty);

                Some(ParameterNode {
                    id: param_id,
                    name,
                    type_id,
                    is_mutable,
                    is_self: false,
                })
            }
            FnArg::Receiver(receiver) => {
                let type_id = self.state_mut().get_or_create_type(&receiver.ty);

                Some(ParameterNode {
                    id: self.next_node_id(),
                    name: Some("self".to_string()),
                    type_id,
                    is_mutable: receiver.mutability.is_some(),
                    is_self: true,
                })
            }
        }
    }
}

// CodeProcessor implementation
impl CodeProcessor for VisitorState {
    type State = Self;

    fn state_mut(&mut self) -> &mut Self::State {
        self
    }
}

// StateManagement implementation
impl StateManagement for VisitorState {
    fn code_graph(&mut self) -> &mut CodeGraph {
        &mut self.code_graph
    }

    fn add_function(&mut self, function: FunctionNode) {
        self.code_graph.functions.push(function);
    }

    fn add_relation(&mut self, relation: Relation) {
        self.code_graph.relations.push(relation);
    }

    fn get_or_create_type(&mut self, ty: &Type) -> TypeId {
        let type_str = ty.to_token_stream().to_string();
        if let Some(&id) = self.type_map.get(&type_str) {
            return id;
        }

        let id = self.next_type_id();
        self.type_map.insert(type_str, id);
        id
    }
    fn next_node_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id.0);
        self.next_node_id.0 += 1;
        id
    }

    fn next_type_id(&mut self) -> TypeId {
        let id = TypeId(self.next_type_id.0);
        self.next_type_id.0 += 1;
        id
    }
}

// TypeOperations implementation
impl TypeOperations for VisitorState {
    // fn get_or_create_type(&mut self, ty: &Type) -> TypeId {
    //     let type_str = ty.to_token_stream().to_string();
    //     if let Some(&id) = self.type_map.get(&type_str) {
    //         return id;
    //     }
    //
    //     let (type_kind, related_types) = self.process_type(ty);
    //     let id = self.next_type_id();
    //     self.type_map.insert(type_str, id);
    //
    //     self.code_graph.type_graph.push(TypeNode {
    //         id,
    //         kind: type_kind,
    //         related_types,
    //     });
    //
    //     id
    // }

    fn process_type(&mut self, ty: &Type) -> (TypeKind, Vec<TypeId>) {
        // This would be the implementation of process_type
        // For now, we'll use a simple placeholder implementation
        match ty {
            Type::Path(type_path) => {
                let path: Vec<String> = type_path
                    .path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect();

                // Process any generic arguments if present
                let mut related_types = Vec::new();
                for seg in &type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(arg_type) = arg {
                                related_types.push(self.state_mut().get_or_create_type(&arg_type));
                            }
                        }
                    }
                }

                (
                    TypeKind::Named {
                        path,
                        is_fully_qualified: type_path.qself.is_some(),
                    },
                    related_types,
                )
            }
            Type::Reference(type_ref) => {
                let inner_type_id = self.state_mut().get_or_create_type(&type_ref.elem);
                let lifetime = type_ref.lifetime.as_ref().map(|l| l.ident.to_string());

                (
                    TypeKind::Reference {
                        lifetime,
                        is_mutable: type_ref.mutability.is_some(),
                    },
                    vec![inner_type_id],
                )
            }
            // Implement other type variants as needed
            unknown_type => (
                TypeKind::Unknown {
                    type_str: unknown_type.to_token_stream().to_string(),
                },
                Vec::new(),
            ),
        }
    }
}

// DocOperations implementation
impl DocOperations for VisitorState {
    fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String> {
        docs::extract_docstring(attrs)
    }
}

// AttributeOperations implementation
impl AttributeOperations for VisitorState {
    fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
        attributes::extract_attributes(attrs)
    }
}

// GenericsOperations implementation
impl GenericsOperations for VisitorState {
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
        crate::parser::visitor::utils::generics::process_generics(self, generics)
    }
}
