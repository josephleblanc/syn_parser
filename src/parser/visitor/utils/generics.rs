use crate::parser::{
    types::{GenericParamNode, TypeId, TypeKind, TypeNode},
    visitor::{
        processor, state::VisitorState, GenericParamKind, GenericsOperations, StateManagement,
    },
};
use quote::ToTokens;
use syn::{Generics, Type, TypeParam, TypePath};

use crate::parser::visitor::CodeProcessor;

/// GenericsProcessor trait for handling generic parameters
/// Builds on top of CodeProcessor for state management
pub trait GenericsProcessor: CodeProcessor
where
    Self::State:
        processor::StateManagement + processor::TypeOperations + processor::GenericsOperations,
{
    /// Process generics from a syn::Generics structure
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
        self.state_mut().process_generics(generics)
    }

    /// Process a type bound like trait bounds in generics
    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId {
        match bound {
            syn::TypeParamBound::Trait(trait_bound) => {
                let ty = Type::Path(TypePath {
                    qself: None,
                    path: trait_bound.path.clone(),
                });
                self.state_mut().get_or_create_type(&ty)
            }
            syn::TypeParamBound::Lifetime(_) => {
                let type_id = self.state_mut().next_type_id();
                self.state_mut().code_graph().type_graph.push(TypeNode {
                    id: type_id,
                    kind: TypeKind::Named {
                        path: vec!["lifetime".to_string()],
                        is_fully_qualified: false,
                    },
                    related_types: Vec::new(),
                });
                type_id
            }
            _ => self.state_mut().next_type_id(),
        }
    }

    /// Process lifetime bounds
    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        bound.ident.to_string()
    }

    /// Process an individual generic parameter
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

                // Placeholder
                // TODO: Figure out what to do with this later
                let default_type = None;
                // let default_type = default.as_ref().map(|expr| {
                // let path = expr.to_token_stream().to_string();
                // let state = self.state_mut();
                // state.type_map.get(&path).cloned().unwrap_or_else(|| {
                //     let id = state.next_type_id();
                //     state.get_or_create_type(expr);
                //     id
                // })
                // });

                GenericParamNode {
                    id: self.state_mut().next_node_id(),
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
                    id: self.state_mut().next_node_id(),
                    kind: GenericParamKind::Lifetime {
                        name: lifetime_def.lifetime.ident.to_string(),
                        bounds,
                    },
                }
            }
            syn::GenericParam::Const(const_param) => {
                let type_id = self.state_mut().get_or_create_type(&const_param.ty);
                GenericParamNode {
                    id: self.state_mut().next_node_id(),
                    kind: GenericParamKind::Const {
                        name: const_param.ident.to_string(),
                        type_id,
                    },
                }
            }
        }
    }
}

// Blanket implementation for all types that implement CodeProcessor
impl<T> GenericsProcessor for T
where
    T: CodeProcessor,
    T::State:
        processor::StateManagement + processor::TypeOperations + processor::GenericsOperations,
{
}

// This is the utility function that's used by VisitorState
// to implement GenericsOperations
pub fn process_generics(state: &mut VisitorState, generics: &Generics) -> Vec<GenericParamNode> {
    let mut params = Vec::new();

    for param in &generics.params {
        params.push(match param {
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

                GenericParamNode {
                    id: state.next_node_id(),
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
                    .map(|bound| state.process_lifetime_bound(bound))
                    .collect();

                GenericParamNode {
                    id: state.next_node_id(),
                    kind: GenericParamKind::Lifetime {
                        name: lifetime_def.lifetime.ident.to_string(),
                        bounds,
                    },
                }
            }
            syn::GenericParam::Const(const_param) => {
                let type_id = state.get_or_create_type(&const_param.ty);
                GenericParamNode {
                    id: state.next_node_id(),
                    kind: GenericParamKind::Const {
                        name: const_param.ident.to_string(),
                        type_id,
                    },
                }
            }
        });
    }

    params
}
