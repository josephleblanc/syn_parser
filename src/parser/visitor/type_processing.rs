use crate::parser::types::{TypeId, TypeKind, TypeNode};
use crate::parser::visitor::processor::{CodeProcessor, StateManagement, TypeOperations};
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, PathArguments, ReturnType, Type,
    TypeParamBound, TypePath, TypeReference,
};

// TypeProcessor trait that builds on top of CodeProcessor and TypeOperations
pub trait TypeProcessor: CodeProcessor 
where
    Self::State: StateManagement + TypeOperations
{
    // Extended type processing functionality can be added here

    // Process a type bound (like trait bounds in generics)
    fn process_type_bound(&mut self, bound: &TypeParamBound) -> TypeId {
        match bound {
            TypeParamBound::Trait(trait_bound) => {
                let ty = Type::Path(TypePath {
                    qself: None,
                    path: trait_bound.path.clone(),
                });
                self.get_or_create_type(&ty)
            }
            TypeParamBound::Lifetime(_) => {
                let type_id = self.state_mut().next_type_id();
                self.state_mut().code_graph.type_graph.push(TypeNode {
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

    // Process a lifetime bound
    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        bound.ident.to_string()
    }

    // Helper for processing complex types
    fn process_complex_type(&mut self, ty: &Type) -> (TypeKind, Vec<TypeId>) {
        let mut related_types = Vec::new();

        match ty {
            Type::Path(TypePath { path, qself }) => {
                let segments: Vec<String> = path
                    .segments
                    .iter()
                    .map(|seg| {
                        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) = &seg.arguments
                        {
                            for arg in args {
                                if let GenericArgument::Type(arg_type) = arg {
                                    related_types.push(self.get_or_create_type(arg_type));
                                }
                            }
                        } else if let PathArguments::Parenthesized(parenthesized) = &seg.arguments {
                            for input in &parenthesized.inputs {
                                related_types.push(self.get_or_create_type(input));
                            }
                            if let ReturnType::Type(_, return_ty) = &parenthesized.output {
                                related_types.push(self.get_or_create_type(return_ty));
                            }
                        }
                        seg.ident.to_string()
                    })
                    .collect();

                (
                    TypeKind::Named {
                        path: segments,
                        is_fully_qualified: qself.is_some(),
                    },
                    related_types,
                )
            }
            Type::Reference(TypeReference {
                elem,
                lifetime,
                mutability,
                ..
            }) => {
                let elem_id = self.get_or_create_type(elem);
                (
                    TypeKind::Reference {
                        lifetime: lifetime.as_ref().map(|lt| lt.ident.to_string()),
                        is_mutable: mutability.is_some(),
                    },
                    vec![elem_id],
                )
            }
            Type::Tuple(tuple) => {
                let elem_ids: Vec<TypeId> = tuple
                    .elems
                    .iter()
                    .map(|elem| self.get_or_create_type(elem))
                    .collect();
                (TypeKind::Tuple {}, elem_ids)
            }
            Type::Array(array) => {
                let elem_id = self.get_or_create_type(&array.elem);
                let size = array.len.to_token_stream().to_string();
                (TypeKind::Array { size: Some(size) }, vec![elem_id])
            }
            Type::Slice(slice) => {
                let elem_id = self.get_or_create_type(&slice.elem);
                (TypeKind::Slice {}, vec![elem_id])
            }
            Type::Never(_) => (TypeKind::Never, Vec::new()),
            Type::Infer(_) => (TypeKind::Inferred, Vec::new()),
            Type::Ptr(ptr) => {
                let pointee_id = self.get_or_create_type(&ptr.elem);
                (
                    TypeKind::RawPointer {
                        is_mutable: ptr.mutability.is_some(),
                    },
                    vec![pointee_id],
                )
            }
            Type::BareFn(bare_fn) => {
                let mut related_ids = Vec::new();
                for input in &bare_fn.inputs {
                    related_ids.push(self.get_or_create_type(&input.ty));
                }
                if let ReturnType::Type(_, return_ty) = &bare_fn.output {
                    related_ids.push(self.get_or_create_type(return_ty));
                }
                (
                    TypeKind::Function {
                        is_unsafe: bare_fn.unsafety.is_some(),
                        is_extern: bare_fn.abi.is_some(),
                        abi: bare_fn.abi.as_ref().map(|abi| {
                            abi.name
                                .as_ref()
                                .map_or("C".to_string(), |name| name.value())
                        }),
                    },
                    related_ids,
                )
            }
            Type::Paren(paren) => {
                let inner_id = self.get_or_create_type(&paren.elem);
                (TypeKind::Paren {}, vec![inner_id])
            }
            Type::TraitObject(trait_obj) => {
                let bound_ids: Vec<TypeId> = trait_obj
                    .bounds
                    .iter()
                    .filter_map(|bound| {
                        if let TypeParamBound::Trait(trait_bound) = bound {
                            let bound_id = self.state_mut().next_type_id();
                            self.state_mut().code_graph.type_graph.push(TypeNode {
                                id: bound_id,
                                kind: TypeKind::Named {
                                    path: trait_bound
                                        .path
                                        .segments
                                        .iter()
                                        .map(|seg| seg.ident.to_string())
                                        .collect(),
                                    is_fully_qualified: false,
                                },
                                related_types: Vec::new(),
                            });
                            Some(bound_id)
                        } else {
                            None
                        }
                    })
                    .collect();
                (
                    TypeKind::TraitObject {
                        dyn_token: trait_obj.dyn_token.is_some(),
                    },
                    bound_ids,
                )
            }
            Type::ImplTrait(impl_trait) => {
                let bound_ids: Vec<TypeId> = impl_trait
                    .bounds
                    .iter()
                    .filter_map(|bound| {
                        if let TypeParamBound::Trait(trait_bound) = bound {
                            let bound_id = self.state_mut().next_type_id();
                            self.state_mut().code_graph.type_graph.push(TypeNode {
                                id: bound_id,
                                kind: TypeKind::Named {
                                    path: trait_bound
                                        .path
                                        .segments
                                        .iter()
                                        .map(|seg| seg.ident.to_string())
                                        .collect(),
                                    is_fully_qualified: false,
                                },
                                related_types: Vec::new(),
                            });
                            Some(bound_id)
                        } else {
                            None
                        }
                    })
                    .collect();
                (TypeKind::ImplTrait {}, bound_ids)
            }
            Type::Macro(type_macro) => (
                TypeKind::Macro {
                    name: type_macro.mac.path.to_token_stream().to_string(),
                    tokens: type_macro.mac.tokens.to_string(),
                },
                Vec::new(),
            ),
            _ => (
                TypeKind::Unknown {
                    type_str: ty.to_token_stream().to_string(),
                },
                Vec::new(),
            ),
        }
    }
}

// Blanket implementation for all types that implement CodeProcessor
impl<T> TypeProcessor for T 
where 
    T: CodeProcessor,
    T::State: StateManagement + TypeOperations
{}
