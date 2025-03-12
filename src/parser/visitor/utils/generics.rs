use crate::parser::types::GenericParamNode;
use crate::parser::types::TypeId;
use crate::parser::visitor::processor::StateManagement;
use crate::parser::visitor::processor::TypeOperations;
use crate::parser::visitor::state::VisitorState;
use crate::parser::visitor::GenericParamKind;
use quote::ToTokens;
use syn::TypePath;
use syn::{GenericParam, Generics, Lifetime, TypeParam};
use syn::{Type, TypeParamBound};

pub trait GenericsProcessor: TypeOperations + CodeProcessor {
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode>;
    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId;
    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String;
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
