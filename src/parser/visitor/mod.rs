use crate::parser::graph::CodeGraph;
use crate::parser::nodes::NodeId;
use crate::parser::nodes::*;
use crate::parser::relations::*;
use crate::parser::types::GenericParamNode;
use crate::parser::types::TypeId;
use crate::parser::types::*;
use processor::TypeOperations;
use syn::Attribute;

pub mod functions;
pub mod modules;
pub mod state;
pub mod structures;
pub mod traits_impls;
pub mod type_processing;

/// Core processor trait with state management
pub trait CodeProcessor: 
    processor::StateManagement +
    processor::TypeOperations +
    processor::AttributeOperations +
    processor::DocOperations +
    processor::GenericsOperations 
{
    type State;
    
    fn state_mut(&mut self) -> &mut Self::State;

    fn convert_visibility(&mut self, vis: &Visibility) -> VisibilityKind {
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
pub mod utils;

use syn::visit;
pub use type_processing::TypeProcessor;
use utils::attributes::AttributeProcessor;
use utils::docs::DocProcessor;
use utils::generics::GenericsProcessor;

// Blanket implementation for processors with TypeOperations state
impl<T> TypeProcessor for T 
where
    T: CodeProcessor + processor::TypeOperations,
    T::State: processor::TypeOperations,
{}

impl<T: CodeProcessor + TypeOperations> GenericsProcessor for T {
    fn process_generics(&mut self, generics: &syn::Generics) -> Vec<GenericParamNode> {
        generics::process_generics(self.state_mut(), generics)
    }

    fn process_type_bound(&mut self, bound: &syn::TypeParamBound) -> TypeId {
        self.state_mut().process_type_bound(bound)
    }

    fn process_lifetime_bound(&mut self, bound: &syn::Lifetime) -> String {
        self.state_mut().process_lifetime_bound(bound)
    }
}

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

    fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
        match arg {
            FnArg::Typed(pat_type) => {
                let param_id = self.next_node_id();
                let param_name = pat_type.pat.to_token_stream().to_string();
                let type_id = self.get_or_create_type(&pat_type.ty);

                Some(ParameterNode {
                    id: param_id,
                    name: Some(param_name),
                    type_id,
                    // placeholder
                    // TODO: handle cases for mutable/immutable parameters and is_self AI!
                    is_mutable: todo!(),
                    is_self: todo!(),
                    // How should we handle this?
                    // is_mutable:,
                    // How should we handle this?
                    // is_self: todo!(),
                })
            }
            _ => None,
        }
    }

impl<'a> CodeProcessor for CodeVisitor<'a> {
    type State = VisitorState;

    fn state_mut(&mut self) -> &mut Self::State {
        self.state
    }
}

pub mod processor {
    pub trait StateManagement {
        fn next_node_id(&mut self) -> crate::parser::nodes::NodeId;
        fn next_type_id(&mut self) -> crate::parser::types::TypeId;
    }

    pub trait TypeOperations {
        fn get_or_create_type(&mut self, ty: &syn::Type) -> crate::parser::types::TypeId;
        fn process_type(&mut self, ty: &syn::Type) -> (crate::parser::types::TypeKind, Vec<crate::parser::types::TypeId>);
    }

    pub trait AttributeOperations {
        fn extract_attributes(&mut self, attrs: &[syn::Attribute]) -> Vec<crate::parser::nodes::ParsedAttribute>;
    }

    pub trait DocOperations {
        fn extract_docstring(&mut self, attrs: &[syn::Attribute]) -> Option<String>;
    }

    pub trait GenericsOperations {
        fn process_generics(&mut self, generics: &syn::Generics) -> Vec<crate::parser::types::GenericParamNode>;
    }
}


pub fn analyze_code(file_path: &Path) -> Result<CodeGraph, syn::Error> {
    let file = syn::parse_file(&std::fs::read_to_string(file_path).unwrap())?;
    let mut visitor_state = VisitorState::new();

    // Create the root module first
    let root_module_id = visitor_state.next_node_id();
    visitor_state.code_graph.modules.push(ModuleNode {
        id: root_module_id,
        name: "root".to_string(),
        visibility: VisibilityKind::Inherited,
        attributes: Vec::new(),
        docstring: None,
        submodules: Vec::new(),
        items: Vec::new(),
        imports: Vec::new(),
        exports: Vec::new(),
    });

    let mut visitor = CodeVisitor::new(&mut visitor_state);
    visitor.visit_file(&file);

    // Add relations between root module and top-level items
    for module in &visitor_state.code_graph.modules {
        if module.id != root_module_id {
            visitor_state.code_graph.relations.push(Relation {
                source: root_module_id,
                target: module.id,
                kind: RelationKind::Contains,
            });
        }
    }

    Ok(visitor_state.code_graph)
}

// State for the visitor
struct VisitorState {
    code_graph: CodeGraph,
    next_node_id: NodeId,
    next_type_id: TypeId,
    // Maps existing types to their IDs to avoid duplication
    type_map: HashMap<String, TypeId>,
}

impl VisitorState {
    fn new() -> Self {
        Self {
            code_graph: CodeGraph {
                functions: Vec::new(),
                defined_types: Vec::new(),
                type_graph: Vec::new(),
                impls: Vec::new(),
                traits: Vec::new(),
                private_traits: Vec::new(),
                relations: Vec::new(),
                modules: Vec::new(),
                values: Vec::new(),
                macros: Vec::new(),
            },
            next_node_id: 0,
            next_type_id: 0,
            type_map: HashMap::new(),
        }
    }

    fn next_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    fn next_type_id(&mut self) -> TypeId {
        let id = self.next_type_id;
        self.next_type_id += 1;
        id
    }

    // Process a function parameter
    // Decide if this needs to be moved for our refactoring of the visitor module AI
    fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
        match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let type_id = self.get_or_create_type(ty);

                // Extract parameter name and mutability
                let (name, is_mutable) = match &**pat {
                    Pat::Ident(PatIdent {
                        ident, mutability, ..
                    }) => (Some(ident.to_string()), mutability.is_some()),
                    _ => (None, false),
                };

                Some(ParameterNode {
                    id: self.next_node_id(),
                    name,
                    type_id,
                    is_mutable,
                    is_self: false,
                })
            }
            FnArg::Receiver(receiver) => {
                // Create a special self type
                let self_type_id = self.next_type_id();
                let mut related_types = Vec::new();

                // If we have an explicit type for self, include it
                // if let Some(ty_box) = receiver.ty {
                let ty_ref: &syn::Type = &receiver.ty; // Dereference the Box to get &syn::Type
                let inner_type_id = self.get_or_create_type(ty_ref);
                related_types.push(inner_type_id);
                // }

                self.code_graph.type_graph.push(TypeNode {
                    id: self_type_id,
                    kind: TypeKind::Named {
                        path: vec!["Self".to_string()],
                        is_fully_qualified: false,
                    },
                    related_types,
                });

                Some(ParameterNode {
                    id: self.next_node_id(),
                    name: Some("self".to_string()),
                    type_id: self_type_id,
                    is_mutable: receiver.mutability.is_some(),
                    is_self: true,
                })
            }
        }
    }

    // Processing now handled in utils modules
}

// Visitor implementation
struct CodeVisitor<'a> {
    state: &'a mut VisitorState,
}

impl<'a> CodeVisitor<'a> {
    fn new(state: &'a mut VisitorState) -> Self {
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
}

impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a>
where
    Self: TypeOperations + DocProcessor + AttributeProcessor + GenericsProcessor,
{
    fn visit_item_fn(&mut self, func: &'ast ItemFn)
    where
        Self: TypeOperations + DocProcessor + AttributeProcessor + GenericsProcessor,
    {
        // Fix lifetime issues AI!
        <Self as FunctionVisitor>::process_function(self, func);
        visit::visit_item_fn(self, func);
    }

    fn visit_item_struct(&mut self, s: &'ast ItemStruct) {
        <Self as StructVisitor>::process_struct(self, s);
        visit::visit_item_struct(self, s);
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        <Self as ImplVisitor>::process_impl(self, i);
        visit::visit_item_impl(self, i);
    }

    fn visit_item_trait(&mut self, t: &'ast ItemTrait) {
        <Self as TraitVisitor>::process_trait(self, t);
        visit::visit_item_trait(self, t);
    }

    fn visit_item_enum(&mut self, e: &'ast ItemEnum) {
        <Self as StructVisitor>::process_enum(self, e);
        visit::visit_item_enum(self, e);
    }

    fn visit_item_union(&mut self, u: &'ast syn::ItemUnion) {
        <Self as StructVisitor>::process_union(self, u);
        visit::visit_item_union(self, u);
    }

    // Visit struct definitions
    // fn visit_item_struct(&mut self, item_struct: &'ast ItemStruct) {
    // ...
    // }
    // moved to src/parser/visitor/structures.rs

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
                    visibility: self.state.convert_visibility(&item_type.vis),
                    type_id,
                    generic_params,
                    attributes,
                    docstring,
                }));

            visit::visit_item_type(self, item_type);
        }
    }

    // Visit union definitions
    // fn visit_item_union(&mut self, item_union: &'ast syn::ItemUnion) {
    // ...
    // }
    // moved to src/parser/visitor/structures.rs

    // Visit constant items
    // This needs to be moved to another module most likely.
    // Decide where this should go (if it fits into our current files)
    // or create a new file and trait for it. AI
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
                visibility: self.state.convert_visibility(&item_const.vis),
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
        visit::visit_item_const(self, item_const);
    }

    // Visit static items
    // This needs to be moved to another module most likely.
    // Decide where this should go (if it fits into our current files)
    // or create a new file and trait for it. AI
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
                visibility: self.state.convert_visibility(&item_static.vis),
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
        visit::visit_item_static(self, item_static);
    }

    // Visit macro definitions (macro_rules!)
    // This needs to be moved to another module most likely.
    // Decide where this should go (if it fits into our current files)
    // or create a new file and trait for it. AI
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
    // This needs to be moved to another module most likely.
    // Decide where this should go (if it fits into our current files)
    // or create a new file and trait for it. AI
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
        visit::visit_macro(self, mac);
    }
}
