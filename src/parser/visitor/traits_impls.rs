use crate::parser::nodes::{FunctionNode, ImplNode, NodeId, TraitNode};
use crate::parser::relations::{Relation, RelationKind};
use crate::parser::types::VisibilityKind;
// use crate::parser::state::Visitor;
use crate::parser::types::TypeId;
use crate::parser::visitor::functions::FunctionVisitor;
use crate::parser::visitor::processor::{
    CodeProcessor, GenericsOperations, StateManagement, TypeOperations,
};
use crate::parser::visitor::type_processing::TypeProcessor;
use quote::ToTokens;
use syn::{ImplItem, ImplItemFn, Item, ItemImpl, ItemTrait, TraitItem, TraitItemFn, Visibility};

use super::{AttributeOperations, DocOperations};

/// Trait for processing trait definitions
///
/// Builds on FunctionVisitor for function processing capabilities
pub trait TraitVisitor: FunctionVisitor {
    /// Process a trait definition
    fn process_trait(&mut self, t: &ItemTrait) {
        let trait_id = self.state_mut().next_node_id();
        let trait_name = t.ident.to_string();
        let visibility = self.convert_visibility(&t.vis);

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&t.generics);

        // Process super traits (bounds)
        let super_traits: Vec<TypeId> = t
            .supertraits
            .iter()
            .map(|bound| self.process_type_bound(bound))
            .collect();

        // Process trait methods
        let methods = self.process_trait_methods(t, trait_id);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&t.attrs);
        let attributes = self.state_mut().extract_attributes(&t.attrs);

        // Create trait node
        let trait_node = TraitNode {
            id: trait_id,
            name: trait_name,
            visibility: visibility.clone(),
            methods,
            generic_params,
            super_traits,
            attributes,
            docstring,
        };

        // Add to code graph - public or private collection based on visibility
        if matches!(visibility, VisibilityKind::Public) {
            self.state_mut().code_graph.traits.push(trait_node);
        } else {
            self.state_mut().code_graph.private_traits.push(trait_node);
        }

        // Create relations for super traits
        for super_trait_id in &super_traits {
            self.state_mut().code_graph.relations.push(Relation {
                source: trait_id,
                target: *super_trait_id,
                kind: RelationKind::Inherits,
            });
        }
    }

    /// Process trait methods
    fn process_trait_methods(&mut self, t: &ItemTrait, trait_id: NodeId) -> Vec<FunctionNode> {
        t.items
            .iter()
            .filter_map(|item| {
                if let TraitItem::Fn(method) = item {
                    Some(self.process_trait_method(method, trait_id))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Process a single trait method
    fn process_trait_method(&mut self, method: &TraitItemFn, trait_id: NodeId) -> FunctionNode {
        let method_id = self.state_mut().next_node_id();
        let method_name = method.sig.ident.to_string();

        // Process method parameters
        let parameters = self.process_parameters(method.sig.inputs.as_ref());

        // Process return type
        let return_type = match &method.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(self.state_mut().get_or_create_type(ty)),
        };

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&method.sig.generics);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&method.attrs);
        let attributes = self.state_mut().extract_attributes(&method.attrs);

        // Extract body if present (default implementation)
        let body = method
            .default
            .as_ref()
            .map(|block| format!("{}", quote::quote!(#block)));

        // Create function node for the method
        FunctionNode {
            id: method_id,
            name: method_name,
            visibility: VisibilityKind::Public, // Methods in traits are always public
            parameters,
            return_type,
            generic_params,
            attributes,
            docstring,
            body,
        }
    }

}

/// Trait for processing impl blocks
///
/// Builds on FunctionVisitor for function processing capabilities
pub trait ImplVisitor: FunctionVisitor {
    /// Process an impl block
    fn process_impl(&mut self, i: &ItemImpl) {
        let impl_id = self.state_mut().next_node_id();

        // Process the self.state_mut() type (the type being implemented)
        let self_type = self.state_mut().get_or_create_type(&i.self_ty);

        // Process the trait being implemented (if any)
        let trait_type = if let Some((_, path, _)) = &i.trait_ {
            // Convert the path to a Type and process it
            let path_str = format!("{}", quote::quote!(#path));
            let ty = syn::parse_str::<syn::Type>(&path_str).ok();
            ty.map(|t| self.state_mut().get_or_create_type(&t))
        } else {
            None
        };

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&i.generics);

        // Process impl methods
        let methods = self.process_impl_methods(i, impl_id, self_type);

        // Create impl node
        let impl_node = ImplNode {
            id: impl_id,
            self_type,
            trait_type,
            methods,
            generic_params,
        };

        // Add to code graph
        self.state_mut().code_graph.impls.push(impl_node);

        // Create relations
        // Self type relation
        self.state_mut().code_graph.relations.push(Relation {
            source: impl_id,
            target: self_type,
            kind: RelationKind::ImplementsFor,
        });

        // Trait relation if present
        if let Some(trait_id) = trait_type {
            self.state_mut().code_graph.relations.push(Relation {
                source: impl_id,
                target: trait_id,
                kind: RelationKind::ImplementsTrait,
            });
        }
    }

    /// Process impl methods
    fn process_impl_methods(
        &mut self,
        i: &ItemImpl,
        impl_id: NodeId,
        self_type: TypeId,
    ) -> Vec<FunctionNode> {
        i.items
            .iter()
            .filter_map(|item| {
                if let ImplItem::Fn(method) = item {
                    Some(self.process_impl_method(method, impl_id))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Process a single impl method
    fn process_impl_method(&mut self, method: &ImplItemFn, impl_id: NodeId) -> FunctionNode {
        let method_id = self.state_mut().next_node_id();
        let method_name = method.sig.ident.to_string();

        // Process method parameters
        let parameters = self.process_parameters(method.sig.inputs.as_ref());

        // Process return type
        let return_type = match &method.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(self.state_mut().get_or_create_type(ty)),
        };

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&method.sig.generics);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&method.attrs);
        let attributes = self.state_mut().extract_attributes(&method.attrs);

        // Extract body
        let body = Some(format!("{}", quote::quote!(#method.block)));

        // Create function node for the method
        FunctionNode {
            id: method_id,
            name: method_name,
            visibility: self.convert_visibility(&method.vis),
            parameters,
            return_type,
            generic_params,
            attributes,
            docstring,
            body,
        }
    }

}

// Blanket implementations
impl<T> TraitVisitor for T where T: FunctionVisitor {}

impl<T> ImplVisitor for T where T: FunctionVisitor {}
