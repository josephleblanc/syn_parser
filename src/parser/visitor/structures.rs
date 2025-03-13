use crate::parser::nodes::{
    EnumNode, FieldNode, NodeId, StructNode, TypeDefNode, UnionNode, VariantNode,
};
use crate::parser::relations::{Relation, RelationKind};
use crate::parser::types::VisibilityKind;
use crate::parser::visitor::processor::{
    CodeProcessor, GenericsOperations, StateManagement, TypeOperations,
};
use crate::parser::visitor::type_processing::TypeProcessor;
use quote::ToTokens;
use syn::{
    Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Item, ItemEnum, ItemStruct, ItemUnion,
    Visibility,
};

use super::{AttributeOperations, DocOperations};

/// Trait for processing struct, enum and union AST nodes
///
/// Builds on top of TypeProcessor for type resolution capabilities
pub trait StructVisitor: TypeProcessor {
    // pub trait StructVisitor: TypeProcessor {
    /// Process a struct definition
    fn process_struct(&mut self, s: &ItemStruct) {
        let struct_id = self.state_mut().next_node_id();
        let struct_name = s.ident.to_string();

        // Process fields
        let fields = self.process_fields(&s.fields, struct_id);

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&s.generics);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&s.attrs);
        let attributes = self.state_mut().extract_attributes(&s.attrs);

        // Create struct node
        let struct_node = StructNode {
            id: struct_id,
            name: struct_name,
            visibility: self.convert_visibility(&s.vis),
            fields,
            generic_params,
            attributes,
            docstring,
        };

        // Add to code graph
        self.state_mut()
            .code_graph()
            .defined_types
            .push(TypeDefNode::Struct(struct_node));
    }

    /// Process an enum definition
    fn process_enum(&mut self, e: &ItemEnum) {
        let enum_id = self.state_mut().next_node_id();
        let enum_name = e.ident.to_string();

        // Process enum variants
        let variants = self.process_variants(&e.variants, enum_id);

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&e.generics);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&e.attrs);
        let attributes = self.state_mut().extract_attributes(&e.attrs);

        // Create enum node
        let enum_node = EnumNode {
            id: enum_id,
            name: enum_name,
            visibility: self.convert_visibility(&e.vis),
            variants,
            generic_params,
            attributes,
            docstring,
        };

        // Add to code graph
        self.state_mut()
            .code_graph()
            .defined_types
            .push(TypeDefNode::Enum(enum_node));
    }

    /// Process a union definition
    fn process_union(&mut self, u: &ItemUnion) {
        let union_id = self.state_mut().next_node_id();
        let union_name = u.ident.to_string();

        // Process fields
        // TODO: Try to remove this clone later
        let fields = self.process_fields(&Into::<Fields>::into(u.fields.clone()), union_id);

        // Process generic parameters
        let generic_params = self.state_mut().process_generics(&u.generics);

        // Extract documentation and attributes
        let docstring = self.state_mut().extract_docstring(&u.attrs);
        let attributes = self.state_mut().extract_attributes(&u.attrs);

        // Create union node
        let union_node = UnionNode {
            id: union_id,
            name: union_name,
            visibility: self.convert_visibility(&u.vis),
            fields,
            generic_params,
            attributes,
            docstring,
        };

        // Add to code graph
        self.state_mut()
            .code_graph()
            .defined_types
            .push(TypeDefNode::Union(union_node));
    }

    /// Process fields of a struct or union
    fn process_fields(&mut self, fields: &Fields, parent_id: NodeId) -> Vec<FieldNode> {
        match fields {
            Fields::Named(fields_named) => self.process_named_fields(fields_named, parent_id),
            Fields::Unnamed(fields_unnamed) => {
                self.process_unnamed_fields(fields_unnamed, parent_id)
            }
            Fields::Unit => Vec::new(),
        }
    }

    /// Process named fields (e.g., struct { field: Type })
    fn process_named_fields(&mut self, fields: &FieldsNamed, parent_id: NodeId) -> Vec<FieldNode> {
        fields
            .named
            .iter()
            .map(|field| {
                let field_id = self.state_mut().next_node_id();
                let field_name = field.ident.as_ref().map(|i| i.to_string());
                let type_id = self.state_mut().get_or_create_type(&field.ty);

                // Create a relation between the field and its type
                self.state_mut().add_relation(Relation {
                    source: field_id,
                    target: type_id,
                    kind: RelationKind::HasType,
                });

                // Extract attributes
                let attributes = self.state_mut().extract_attributes(&field.attrs);

                FieldNode {
                    id: field_id.into(),
                    name: field_name,
                    type_id,
                    visibility: self.convert_visibility(&field.vis),
                    attributes,
                }
            })
            .collect()
    }

    /// Process unnamed fields (e.g., struct(Type, Type))
    fn process_unnamed_fields(
        &mut self,
        fields: &FieldsUnnamed,
        parent_id: NodeId,
    ) -> Vec<FieldNode> {
        fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let field_id = self.state_mut().next_node_id();
                let type_id = self.state_mut().get_or_create_type(&field.ty);

                // Create a relation between the field and its type
                self.state_mut().add_relation(Relation {
                    source: field_id,
                    target: type_id,
                    kind: RelationKind::HasType,
                });

                // Extract attributes
                let attributes = self.state_mut().extract_attributes(&field.attrs);

                FieldNode {
                    id: field_id,
                    name: Some(format!("{}", idx)),
                    type_id,
                    visibility: self.convert_visibility(&field.vis),
                    attributes,
                }
            })
            .collect()
    }

    /// Process enum variants
    fn process_variants(
        &mut self,
        variants: &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
        enum_id: NodeId,
    ) -> Vec<VariantNode> {
        variants
            .iter()
            .map(|variant| {
                let variant_id = self.state_mut().next_node_id();
                let variant_name = variant.ident.to_string();

                // Process fields
                let fields = self.process_fields(&variant.fields, variant_id);

                // Get discriminant if present
                let discriminant = variant
                    .discriminant
                    .as_ref()
                    .map(|(_, expr)| format!("{}", quote::quote!(#expr)));

                // Extract attributes
                let attributes = self.state_mut().extract_attributes(&variant.attrs);

                VariantNode {
                    id: variant_id,
                    name: variant_name,
                    fields,
                    discriminant,
                    attributes,
                }
            })
            .collect()
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
impl<T> StructVisitor for T where T: TypeProcessor {}
