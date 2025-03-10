use super::state::VisitorState;
use crate::parser::nodes::TypeId;
use crate::parser::types::{TypeKind, TypeNode};
use syn::{Type, TypePath, TypeReference, PathArguments, ReturnType, GenericArgument, AngleBracketedGenericArguments};
use syn::spanned::Spanned;

pub trait TypeProcessor {
    fn get_or_create_type(&mut self, ty: &Type) -> TypeId;
    fn process_type(&mut self, ty: &Type) -> (TypeKind, Vec<TypeId>);
    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId;
    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String;
}

impl TypeProcessor for VisitorState {
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

    fn process_type(&mut self, ty: &Type) -> (TypeKind, Vec<TypeId>) {
        let mut related_types = Vec::new();

        match ty {
            Type::Path(TypePath { path, qself }) => {
                let segments: Vec<String> = path.segments.iter().map(|seg| {
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &seg.arguments {
                        for arg in args {
                            if let GenericArgument::Type(arg_type) = arg {
                                related_types.push(self.get_or_create_type(arg_type));
                            }
                        }
                    }
                    seg.ident.to_string()
                }).collect();

                (TypeKind::Named {
                    path: segments,
                    is_fully_qualified: qself.is_some(),
                }, related_types)
            }
            Type::Reference(TypeReference { elem, lifetime, mutability, .. }) => {
                let elem_id = self.get_or_create_type(elem);
                (TypeKind::Reference {
                    lifetime: lifetime.as_ref().map(|lt| lt.ident.to_string()),
                    is_mutable: mutability.is_some(),
                }, vec![elem_id])
            }
            // ... other type variants handled similarly ...
            _ => (TypeKind::Unknown { type_str: ty.to_token_stream().to_string() }, Vec::new())
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
            _ => self.next_type_id()
        }
    }

    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        bound.ident.to_string()
    }
}
