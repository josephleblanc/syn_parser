use crate::parser::graph::CodeGraph;
use crate::parser::nodes::NodeId;
use crate::parser::nodes::*;
use crate::parser::relations::Relation;
use crate::parser::relations::*;
use crate::parser::types::GenericParamNode;
use crate::parser::types::TypeId;
pub use crate::parser::types::TypeKind;
use crate::parser::types::*;
use crate::parser::visitor::utils::attributes::ParsedAttribute;

pub mod functions;
pub mod modules;
pub mod state;
pub mod structures;
pub mod traits_impls;
pub mod type_processing;

/// Core processor trait with state management
pub mod processor {
    use crate::parser::nodes::{FunctionNode, NodeId};
    use crate::parser::types::{GenericParamNode, TypeId, TypeKind};
    use crate::parser::visitor::utils::attributes::ParsedAttribute;
    use crate::parser::visitor::Relation;
    use crate::CodeGraph;

    pub trait CodeProcessor {
        type State: StateManagement
            + TypeOperations
            + AttributeOperations
            + DocOperations
            + GenericsOperations;

        fn state_mut(&mut self) -> &mut Self::State;
    }

    pub trait StateManagement {
        fn next_node_id(&mut self) -> NodeId;
        fn next_type_id(&mut self) -> TypeId;
        fn code_graph(&mut self) -> &mut CodeGraph;
        fn add_function(&mut self, function: FunctionNode);
        fn add_relation(&mut self, relation: Relation);
        fn get_or_create_type(&mut self, ty: &syn::Type) -> TypeId;
    }

    pub trait TypeOperations {
        fn process_type(&mut self, ty: &syn::Type) -> (TypeKind, Vec<TypeId>);
    }

    pub trait AttributeOperations {
        fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<ParsedAttribute>;
    }

    pub trait DocOperations {
        fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String>;
    }

    pub trait GenericsOperations {
        fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode>;
    }
}
pub mod utils;

// Blanket implementations for CodeProcessor
// impl<T: CodeProcessor> StateManagement for T {
//     fn next_node_id(&mut self) -> NodeId {
//         self.state_mut().next_node_id()
//     }
//
//     fn next_type_id(&mut self) -> TypeId {
//         self.state_mut().next_type_id()
//     }
// }

// impl<T: CodeProcessor> TypeOperations for T {
//     fn get_or_create_type(&mut self, ty: &syn::Type) -> TypeId {
//         self.state_mut().get_or_create_type(ty)
//     }
//
//     fn process_type(&mut self, ty: &syn::Type) -> (TypeKind, Vec<TypeId>) {
//         self.state_mut().process_type(ty)
//     }
// }

// impl<T: CodeProcessor> AttributeOperations for T {
//     fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<ParsedAttribute> {
//         self.state_mut().extract_attributes(attrs)
//     }
// }

// impl<T: CodeProcessor> DocOperations for T {
//     fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String> {
//         self.state_mut().extract_docstring(attrs)
//     }
// }
//
// impl<T: CodeProcessor> GenericsOperations for T {
//     fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
//         self.state_mut().process_generics(generics)
//     }
// }

// Re-export operation traits from processor module
pub use processor::{
    AttributeOperations, CodeProcessor, DocOperations, GenericsOperations, StateManagement,
    TypeOperations,
};

// Re-export types used in processor traits

use self::utils::{attributes, docs, generics};
pub use self::{
    functions::FunctionVisitor,
    modules::ModuleVisitor,
    structures::StructVisitor,
    traits_impls::{ImplVisitor, TraitVisitor},
};

use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use syn::{
    visit::Visit, FnArg, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, Pat, PatIdent, PatType,
    Visibility,
};

pub fn analyze_code(file_path: &Path) -> Result<CodeGraph, syn::Error> {
    let file = syn::parse_file(&std::fs::read_to_string(file_path).unwrap())?;
    let mut visitor_state = state::VisitorState::new();

    // Create the root module first
    let root_module_id = visitor_state.next_node_id();
    let root_module = ModuleNode {
        id: root_module_id,
        name: "root".to_string(),
        visibility: VisibilityKind::Inherited,
        attributes: Vec::new(),
        docstring: None,
        submodules: Vec::new(),
        items: Vec::new(),
        imports: Vec::new(),
        exports: Vec::new(),
    };
    visitor_state.code_graph.modules.push(root_module);

    let mut visitor = CodeVisitor::new(&mut visitor_state);
    visitor.visit_file(&file);

    // Process top-level modules after visiting
    // First collect module names and IDs
    let module_entries: Vec<(String, NodeId)> = visitor_state.code_graph.modules
        .iter()
        .map(|m| (m.name.clone(), m.id))
        .collect();
    
    // Then update root module relationships
    if let Some(root_module) = visitor_state.code_graph.modules.first_mut() {
        for item in &file.items {
            if let syn::Item::Mod(item_mod) = item {
                if let Some((_, module_id)) = module_entries.iter()
                    .find(|(name, _)| name == &item_mod.ident.to_string()) 
                {
                    root_module.submodules.push(*module_id);
                    visitor_state.code_graph.relations.push(Relation {
                        source: root_module_id,
                        target: *module_id,
                        kind: RelationKind::Contains,
                    });
                }
            }
        }
    }

    Ok(visitor_state.code_graph)
}

// Visitor implementation
pub struct CodeVisitor<'a> {
    state: &'a mut state::VisitorState,
}

impl<'a> CodeProcessor for CodeVisitor<'a> {
    type State = state::VisitorState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}

impl<'a> CodeVisitor<'a> {
    fn new(state: &'a mut state::VisitorState) -> Self {
        Self { state }
    }

    // Helper method to extract path segments from a use tree
    fn extract_use_path(use_tree: &syn::UseTree, path_segments: &mut Vec<String>) {
        match use_tree {
            syn::UseTree::Path(path) => {
                path_segments.push(path.ident.to_string());
                CodeVisitor::extract_use_path(&path.tree, path_segments);
            }
            syn::UseTree::Name(name) => {
                path_segments.push(name.ident.to_string());
            }
            syn::UseTree::Rename(rename) => {
                path_segments.push(format!("{} as {}", rename.ident, rename.rename));
            }
            syn::UseTree::Glob(_) => {
                path_segments.push("*".to_string());
            }
            syn::UseTree::Group(group) => {
                for tree in &group.items {
                    let mut new_path = path_segments.clone();
                    CodeVisitor::extract_use_path(tree, &mut new_path);
                }
            }
        }
    }

    // Add visibility conversion as a method to align with trait-based architecture
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

impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    fn visit_item_fn(&mut self, func: &'ast ItemFn) {
        <Self as FunctionVisitor>::process_function(self, func);
        syn::visit::visit_item_fn(self, func);
    }

    fn visit_item_struct(&mut self, s: &'ast ItemStruct) {
        <Self as StructVisitor>::process_struct(self, s);
        syn::visit::visit_item_struct(self, s);
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        <Self as ImplVisitor>::process_impl(self, i);
        syn::visit::visit_item_impl(self, i);
    }

    fn visit_item_trait(&mut self, t: &'ast ItemTrait) {
        <Self as TraitVisitor>::process_trait(self, t);
        syn::visit::visit_item_trait(self, t);
    }

    fn visit_item_enum(&mut self, e: &'ast ItemEnum) {
        <Self as StructVisitor>::process_enum(self, e);
        syn::visit::visit_item_enum(self, e);
    }

    fn visit_item_union(&mut self, u: &'ast syn::ItemUnion) {
        <Self as StructVisitor>::process_union(self, u);
        syn::visit::visit_item_union(self, u);
    }

    // Visit type alias definitions
    fn visit_item_type(&mut self, item_type: &'ast syn::ItemType) {
        let type_alias_id = self.state.next_node_id();
        let type_alias_name = item_type.ident.to_string();

        // Process the aliased type
        let type_id = self.state.get_or_create_type(&item_type.ty);

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_type.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_type.attrs);
        let attributes = self.state.extract_attributes(&item_type.attrs);

        // Store type alias info only if public
        if matches!(item_type.vis, Visibility::Public(_)) {
            self.state
                .code_graph
                .defined_types
                .push(TypeDefNode::TypeAlias(TypeAliasNode {
                    id: type_alias_id,
                    name: type_alias_name,
                    visibility: self.convert_visibility(&item_type.vis),
                    type_id,
                    generic_params,
                    attributes,
                    docstring,
                }));

            syn::visit::visit_item_type(self, item_type);
        }
    }

    // Visit constant items
    fn visit_item_const(&mut self, item_const: &'ast syn::ItemConst) {
        // Check if the constant is public
        if matches!(item_const.vis, Visibility::Public(_)) {
            let const_id = self.state.next_node_id();
            let const_name = item_const.ident.to_string();

            // Process the type
            let type_id = self.state.get_or_create_type(&item_const.ty);

            // Extract the value expression as a string
            let value = Some(item_const.expr.to_token_stream().to_string());

            // Extract doc comments and other attributes
            let docstring = self.state.extract_docstring(&item_const.attrs);
            let attributes = self.state.extract_attributes(&item_const.attrs);

            // Create the constant node
            let const_node = ValueNode {
                id: const_id,
                name: const_name,
                visibility: self.convert_visibility(&item_const.vis),
                type_id,
                kind: ValueKind::Constant,
                value,
                attributes,
                docstring,
            };

            // Add the constant to the code graph
            self.state.code_graph.values.push(const_node);

            // Add relation between constant and its type
            self.state.code_graph.relations.push(Relation {
                source: const_id,
                target: type_id,
                kind: RelationKind::ValueType,
            });
        }

        // Continue visiting
        syn::visit::visit_item_const(self, item_const);
    }

    // Visit static items
    fn visit_item_static(&mut self, item_static: &'ast syn::ItemStatic) {
        // Check if the static variable is public
        if matches!(item_static.vis, Visibility::Public(_)) {
            let static_id = self.state.next_node_id();
            let static_name = item_static.ident.to_string();

            // Process the type
            let type_id = self.state.get_or_create_type(&item_static.ty);

            // Extract the value expression as a string
            let value = Some(item_static.expr.to_token_stream().to_string());

            // Extract doc comments and other attributes
            let docstring = self.state.extract_docstring(&item_static.attrs);
            let attributes = self.state.extract_attributes(&item_static.attrs);

            // Create the static node
            let static_node = ValueNode {
                id: static_id,
                name: static_name,
                visibility: self.convert_visibility(&item_static.vis),
                type_id,
                kind: ValueKind::Static {
                    is_mutable: matches!(item_static.mutability, syn::StaticMutability::Mut(_)),
                },
                value,
                attributes,
                docstring,
            };

            // Add the static to the code graph
            self.state.code_graph.values.push(static_node);

            // Add relation between static and its type
            self.state.code_graph.relations.push(Relation {
                source: static_id,
                target: type_id,
                kind: RelationKind::ValueType,
            });
        }

        // Continue visiting
        syn::visit::visit_item_static(self, item_static);
    }

    // Visit macro definitions (macro_rules!)
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        let module_id = self.state.next_node_id();
        let module_name = m.ident.to_string();
        
        // Create and store the module node
        let module = ModuleNode {
            id: module_id,
            name: module_name.clone(),
            visibility: self.convert_visibility(&m.vis),
            attributes: self.state.extract_attributes(&m.attrs),
            docstring: self.state.extract_docstring(&m.attrs),
            submodules: Vec::new(),
            items: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        };
        self.state.code_graph.modules.push(module);

        // Continue visiting into the module content
        if let Some((_, items)) = &m.content {
            for item in items {
                self.visit_item(item);
            }
        }
    }

    fn visit_item_macro(&mut self, item_macro: &'ast syn::ItemMacro) {
        // Only process macros with #[macro_export]
        if !item_macro
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("macro_export"))
        {
            return;
        }

        let macro_id = self.state.next_node_id();

        // Get the macro name
        let macro_name = item_macro
            .ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or_else(|| "unnamed_macro".to_string());

        // Extract the macro body
        let body = Some(item_macro.mac.tokens.to_string());

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_macro.attrs);
        let attributes = self.state.extract_attributes(&item_macro.attrs);

        // Parse macro rules (simplified approach)
        let mut rules = Vec::new();
        let tokens_str = item_macro.mac.tokens.to_string();

        // Very basic parsing of macro rules - in a real implementation,
        // you would want to use a more sophisticated approach
        for (_i, rule) in tokens_str.split(";").enumerate() {
            if rule.trim().is_empty() {
                continue;
            }

            // Try to split the rule into pattern and expansion
            if let Some(idx) = rule.find("=>") {
                let pattern = rule[..idx].trim().to_string();
                let expansion = rule[(idx + 2)..].trim().to_string();

                rules.push(MacroRuleNode {
                    id: self.state.next_node_id(),
                    pattern,
                    expansion,
                });
            }
        }

        // Create the macro node
        let macro_node = MacroNode {
            id: macro_id,
            name: macro_name,
            visibility: VisibilityKind::Public, // Macros with #[macro_export] are public
            kind: MacroKind::DeclarativeMacro,
            rules,
            attributes,
            docstring,
            body,
            expansion: None,
            parent_function: None,
        };

        // Add the macro to the code graph
        self.state.code_graph.macros.push(macro_node);
    }

    // Visit macro invocations
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        // Create a node ID for this macro invocation
        let invocation_id = self.state.next_node_id();

        // Get the macro name
        let macro_path = mac.path.to_token_stream().to_string();

        // Find if this macro is defined in our code graph
        let defined_macro = self
            .state
            .code_graph
            .macros
            .iter()
            .find(|m| m.name == macro_path.split("::").last().unwrap_or(&macro_path));

        if let Some(defined_macro) = defined_macro {
            // Add a relation between the invocation and the macro definition
            self.state.code_graph.relations.push(Relation {
                source: invocation_id,
                target: defined_macro.id,
                kind: RelationKind::MacroUse,
            });
        }

        // Continue visiting
        syn::visit::visit_macro(self, mac);
    }
}
