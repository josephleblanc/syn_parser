use std::ops::Deref;

use crate::parser::nodes::{FunctionNode, NodeId, ParameterNode};
use crate::parser::relations::{Relation, RelationKind};
use crate::parser::types::TypeId;
use crate::parser::types::VisibilityKind;
use crate::parser::visitor::processor::{CodeProcessor, StateManagement, TypeOperations};
use crate::parser::visitor::type_processing::TypeProcessor;
use quote::ToTokens;
use syn::{FnArg, ItemFn, PatType, ReturnType, Signature, Type, Visibility};

use super::{AttributeOperations, DocOperations, GenericsOperations};

/// Trait for processing function-related AST nodes
///
/// Builds on top of TypeProcessor for type resolution capabilities
pub trait FunctionVisitor: TypeProcessor {
    /// Process a function definition
    fn process_function(&mut self, func: &ItemFn) {
        let fn_id = self.state_mut().next_node_id();
        let fn_name = func.sig.ident.to_string();
        let visibility = self.convert_visibility(&func.vis);

        // Process function parameters
        // This method is giving an error. AI!
        let parameters = self.process_parameters(func.sig.inputs.iter().collect::<Vec<_>>().as_slice());

        // Process return type
        let return_type = match &func.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ref ty) => Some(self.state_mut().get_or_create_type(ty)),
        };

        // Process generic parameters if any
        let generic_params = self.state_mut().process_generics(&func.sig.generics);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&func.attrs);
        let attributes = self.state_mut().extract_attributes(&func.attrs);

        // Extract function body if we need it
        let body = Some(func.block.to_token_stream().to_string());

        // Create function node
        let function_node = FunctionNode {
            id: fn_id,
            name: fn_name,
            visibility,
            parameters,
            return_type,
            generic_params,
            attributes,
            docstring,
            body,
        };

        // Add to code graph
        self.state_mut().add_function(function_node);

        // Create relations for parameter types and return type
        self.create_function_relations(fn_id, &parameters, return_type);
    }

    /// Process function parameters
    // This function is not being called correctly elsewhere, lets decide how to handle the
    // `params` parameter.
    fn process_parameters(&mut self, params: &[FnArg]) -> Vec<ParameterNode> {
        params
            .iter()
            .filter_map(|arg| self.process_fn_arg(arg))
            .collect()
    }

    /// Process a single function argument
    fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
        match arg {
            FnArg::Typed(pat_type) => {
                let param_id = self.state_mut().next_node_id();
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
                let param_id = self.state_mut().next_node_id();
                // For self parameters, create a special parameter node
                let type_id = if let Some(ty) = &receiver.ty {
                    self.state_mut().get_or_create_type(ty)
                } else {
                    // Create a placeholder type for self
                    self.state_mut().next_type_id()
                    // Ideally we would resolve to the impl type here
                };

                Some(ParameterNode {
                    id: param_id,
                    name: Some("self".to_string()),
                    type_id,
                    is_mutable: receiver.mutability.is_some(),
                    is_self: true,
                })
            }
        }
    }

    /// Create relations between function and its parameter/return types
    fn create_function_relations(
        &mut self,
        fn_id: NodeId,
        parameters: &[ParameterNode],
        return_type: Option<TypeId>,
    ) {
        // Add relations for parameter types
        for param in parameters {
            self.state_mut().code_graph.relations.push(Relation {
                source: fn_id,
                target: param.type_id,
                kind: RelationKind::Uses,
            });
        }

        // Add relation for return type
        if let Some(type_id) = return_type {
            self.state_mut().code_graph.relations.push(Relation {
                source: fn_id,
                target: type_id,
                kind: RelationKind::Returns,
            });
        }
    }

    /// Convert visibility modifier to our internal representation
    fn convert_visibility(&self, vis: &Visibility) -> VisibilityKind {
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

// Blanket implementation for all types that implement TypeProcessor
impl<T> FunctionVisitor for T where T: TypeProcessor {}
