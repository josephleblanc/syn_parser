use crate::parser::types::{GenericParamNode, TypeId, TypeNode, TypeKind};
use crate::parser::visitor::processor;
use crate::parser::visitor::state::VisitorState;
use crate::parser::visitor::GenericParamKind;
use quote::ToTokens;
use syn::TypePath;
use syn::{GenericParam, Generics, Lifetime, TypeParam};
use syn::{Type, TypeParamBound};

use crate::parser::visitor::CodeProcessor;

pub trait GenericsProcessor {
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode>;
    fn process_generic_param(&mut self, param: &syn::GenericParam) -> GenericParamNode;
    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId;
    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String;
}

impl<T> GenericsProcessor for T
where
    T: CodeProcessor + processor::TypeOperations + processor::StateManagement
{
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
        let state = self.state_mut();
        process_generics(state, generics)
    }
    
    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId {
        self.state_mut().process_type_bound(bound)
    }

    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        self.state_mut().process_lifetime_bound(bound)
    }
}

impl GenericsProcessor for VisitorState {
    fn process_generic_param(&mut self, param: &syn::GenericParam) -> GenericParamNode {
        match param {
            syn::GenericParam::Type(TypeParam {
                ident,
                bounds,
                default,
                ..
            }) => {
                let bounds: Vec<_> = bounds
                    .iter()
                    .map(|bound| self.process_type_bound(bound))
                    .collect();

                let default_type = default.as_ref().map(|expr| {
                    let path = expr.to_token_stream().to_string();
                    self.type_map.get(&path).cloned().unwrap_or_else(|| {
                        let id = self.next_type_id();
                        self.get_or_create_type(expr);
                        id
                    })
                });

                GenericParamNode {
                    id: self.next_node_id(),
                    kind: GenericParamKind::Type {
                        name: ident.to_string(),
                        bounds,
                        default: default_type,
                    },
                }
            }
            syn::GenericParam::Lifetime(lifetime_def) => {
                let bounds: Vec<String> = lifetime_def
                    .bounds
                    .iter()
                    .map(|bound| self.process_lifetime_bound(bound))
                    .collect();

                GenericParamNode {
                    id: self.next_node_id(),
                    kind: GenericParamKind::Lifetime {
                        name: lifetime_def.lifetime.ident.to_string(),
                        bounds,
                    },
                }
            }
            syn::GenericParam::Const(const_param) => {
                let type_id = self.get_or_create_type(&const_param.ty);
                GenericParamNode {
                    id: self.next_node_id(),
                    kind: GenericParamKind::Const {
                        name: const_param.ident.to_string(),
                        type_id,
                    },
                }
            }
        }
    }
    
    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId {
        match bound {
            syn::TypeParamBound::Trait(trait_bound) => {
                let ty = Type::Path(TypePath {
                    qself: None,
                    path: trait_bound.path.clone(),
                });
                self.get_or_create_type(&ty)
            }
            syn::TypeParamBound::Lifetime(_) => {
                let type_id = self.next_type_id();
                self.code_graph.type_graph.push(TypeNode {
                    id: type_id,
                    kind: TypeKind::Named {
                        path: vec!["lifetime".to_string()],
                        is_fully_qualified: false,
                    },
                    related_types: Vec::new(),
                });
                type_id
            }
            _ => self.next_type_id(),
        }
    }
    
    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        bound.ident.to_string()
    }
}

pub fn process_generics(state: &mut VisitorState, generics: &Generics) -> Vec<GenericParamNode> {
    let mut params = Vec::new();

    for param in &generics.params {
        match param {
            syn::GenericParam::Type(TypeParam {
                ident,
                bounds,
                default,
                ..
            }) => {
                let bounds: Vec<_> = bounds
                    .iter()
                    .map(|bound| state.process_type_bound(bound))
                    .collect();

                let default_type = default.as_ref().map(|expr| {
                    let path = expr.to_token_stream().to_string();
                    state.type_map.get(&path).cloned().unwrap_or_else(|| {
                        let id = state.next_type_id();
                        state.get_or_create_type(expr);
                        id
                    })
                });

                params.push(GenericParamNode {
                    id: state.next_node_id(),
                    kind: GenericParamKind::Type {
                        name: ident.to_string(),
                        bounds,
                        default: default_type,
                    },
                });
            }
            syn::GenericParam::Lifetime(lifetime_def) => {
                let bounds: Vec<String> = lifetime_def
                    .bounds
                    .iter()
                    .map(|bound| state.process_lifetime_bound(bound))
                    .collect();

                params.push(GenericParamNode {
                    id: state.next_node_id(),
                    kind: GenericParamKind::Lifetime {
                        name: lifetime_def.lifetime.ident.to_string(),
                        bounds,
                    },
                });
            }
            syn::GenericParam::Const(const_param) => {
                let type_id = state.get_or_create_type(&const_param.ty);
                params.push(GenericParamNode {
                    id: state.next_node_id(),
                    kind: GenericParamKind::Const {
                        name: const_param.ident.to_string(),
                        type_id,
                    },
                });
            }
        }
    }

    params
}
