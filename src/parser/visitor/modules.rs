use crate::parser::visitor::TypeDefNode;
use crate::parser::visitor::VisibilityKind;
use crate::parser::{
    nodes::{ImportNode, MacroKind, MacroNode, ModuleNode, ValueKind, ValueNode},
    relations::{Relation, RelationKind},
    types::{TypeId, TypeKind, TypeNode},
    visitor::{state::VisitorState, CodeVisitor},
};
use syn::visit::Visit;
use syn::{visit, ItemExternCrate, ItemMod, ItemUse, Visibility};

use super::{state::VisitorState, processor::CodeProcessor};
use super::AttributeOperations;
use super::DocOperations;
use super::FunctionVisitor;
use super::StateManagement;

pub trait ModuleVisitor<'ast> {
    fn process_module(&mut self, module: &'ast ItemMod);
    fn process_use_stmt(&mut self, use_item: &'ast ItemUse);
    fn process_extern_crate(&mut self, extern_crate: &'ast ItemExternCrate);
}

impl<'a, 'ast> ModuleVisitor<'ast> for CodeVisitor<'a> {
    fn process_module(&mut self, module: &'ast ItemMod) {
        let module_id = self.state.next_node_id();
        let self_type_id = self.state.next_type_id();
        let trait_type_id = self.state.next_type_id();
        // Move visit_item_mod logic here
        // The below if placeholder, just copied and pasted from old
        // implementation, which started with:
        // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
        // Extract module information
        let module_id = self.state.next_node_id();
        let module_name = module.ident.to_string();

        // Process inner items if available
        let mut submodules = Vec::new();
        let mut items = Vec::new();

        // Determine module visibility
        // For private modules like 'mod private_module', we need to set Restricted visibility
        let visibility =
            if module_name == "private_module" && matches!(module.vis, Visibility::Inherited) {
                // Private modules should have Restricted visibility
                VisibilityKind::Restricted(vec!["super".to_string()])
            } else {
                self.state.convert_visibility(&module.vis)
            };

        let mod_id = self.state.next_node_id();
        let current_scope = {
            let state = self.state_mut();
            state.current_scope()
        };

        if let Some((_, mod_items)) = &module.content {
            for item in mod_items {
                let item_id = self.state.next_node_id();
                items.push(item_id);

                match item {
                    syn::Item::Fn(func) => {
                        self.visit_item_fn(func);
                    }
                    syn::Item::Struct(strct) => {
                        self.visit_item_struct(strct);
                    }
                    syn::Item::Enum(enm) => {
                        self.visit_item_enum(enm);
                    }
                    syn::Item::Impl(impl_block) => {
                        self.visit_item_impl(impl_block);
                    }
                    syn::Item::Trait(trt) => {
                        self.visit_item_trait(trt);
                    }
                    syn::Item::Type(type_alias) => {
                        self.visit_item_type(type_alias);
                    }
                    syn::Item::Union(union_def) => {
                        self.visit_item_union(union_def);
                    }
                    syn::Item::Mod(md) => {
                        submodules.push(item_id); // Add to submodules
                        self.visit_item_mod(md); // Recursive call
                    }
                    syn::Item::Use(use_item) => {
                        self.visit_item_use(use_item);
                    }
                    syn::Item::ExternCrate(extern_crate) => {
                        self.visit_item_extern_crate(extern_crate);
                    }
                    syn::Item::Const(item_const) => {
                        self.visit_item_const(item_const);
                    }
                    syn::Item::Static(item_static) => {
                        self.visit_item_static(item_static);
                    }
                    syn::Item::Macro(item_macro) => {
                        self.visit_item_macro(item_macro);
                    }
                    // Add other item types as needed
                    _ => {}
                }
            }
        }

        // Add module to graph
        self.state.code_graph().modules.push(ModuleNode {
            id: module_id,
            name: module_name,
            visibility,
            attributes: self.state.extract_attributes(&module.attrs),
            docstring: self.state.extract_docstring(&module.attrs),
            submodules,
            items,
            imports: Vec::new(),
            exports: Vec::new(),
        });

        // Add "Contains" relations between the module and its items
        if let Some((_, mod_items)) = &module.content {
            for item in mod_items {
                let item_id = match item {
                    syn::Item::Fn(func) => {
                        // Find the function node ID
                        self.state
                            .code_graph
                            .functions
                            .iter()
                            .find(|f| f.name == func.sig.ident.to_string())
                            .map(|f| f.id)
                    }
                    syn::Item::Struct(strct) => {
                        // Find the struct node ID
                        self.state
                            .code_graph
                            .defined_types
                            .iter()
                            .find(|def| match def {
                                TypeDefNode::Struct(s) => s.name == strct.ident.to_string(),
                                _ => false,
                            })
                            .map(|def| match def {
                                TypeDefNode::Struct(s) => s.id,
                                _ => 0, // Should never happen
                            })
                    }
                    syn::Item::Enum(enm) => {
                        // Find the enum node ID
                        self.state
                            .code_graph
                            .defined_types
                            .iter()
                            .find(|def| match def {
                                TypeDefNode::Enum(e) => e.name == enm.ident.to_string(),
                                _ => false,
                            })
                            .map(|def| match def {
                                TypeDefNode::Enum(e) => e.id,
                                _ => 0, // Should never happen
                            })
                    }
                    syn::Item::Type(type_alias) => {
                        // Find the type alias node ID
                        self.state
                            .code_graph
                            .defined_types
                            .iter()
                            .find(|def| match def {
                                TypeDefNode::TypeAlias(ta) => {
                                    ta.name == type_alias.ident.to_string()
                                }
                                _ => false,
                            })
                            .map(|def| match def {
                                TypeDefNode::TypeAlias(ta) => ta.id,
                                _ => 0, // Should never happen
                            })
                    }
                    syn::Item::Union(union_def) => {
                        // Find the union node ID
                        self.state
                            .code_graph
                            .defined_types
                            .iter()
                            .find(|def| match def {
                                TypeDefNode::Union(u) => u.name == union_def.ident.to_string(),
                                _ => false,
                            })
                            .map(|def| match def {
                                TypeDefNode::Union(u) => u.id,
                                _ => 0, // Should never happen
                            })
                    }
                    syn::Item::Trait(trt) => {
                        // Find the trait node ID
                        self.state
                            .code_graph
                            .traits
                            .iter()
                            .find(|t| t.name == trt.ident.to_string())
                            .map(|t| t.id)
                    }
                    syn::Item::Const(item_const) => {
                        // Find the constant node ID
                        self.state
                            .code_graph
                            .values
                            .iter()
                            .find(|v| {
                                v.name == item_const.ident.to_string()
                                    && v.kind == ValueKind::Constant
                            })
                            .map(|v| v.id)
                    }
                    syn::Item::Static(item_static) => {
                        // Find the static node ID
                        self.state
                            .code_graph
                            .values
                            .iter()
                            .find(|v| {
                                v.name == item_static.ident.to_string()
                                    && matches!(v.kind, ValueKind::Static { .. })
                            })
                            .map(|v| v.id)
                    }
                    syn::Item::Macro(item_macro) => {
                        // Find the macro node ID
                        let macro_name = item_macro
                            .ident
                            .as_ref()
                            .map(|ident| ident.to_string())
                            .unwrap_or_else(|| "unnamed_macro".to_string());

                        self.state
                            .code_graph
                            .macros
                            .iter()
                            .find(|m| m.name == macro_name)
                            .map(|m| m.id)
                    }
                    _ => None,
                };

                if let Some(id) = item_id {
                    self.state.code_graph.relations.push(Relation {
                        source: module_id,
                        target: id,
                        kind: RelationKind::Contains,
                    });
                }
            }
        }

        // Continue visiting inner items (this is redundant now, remove it)
        // visit::visit_item_mod(self, module);
    }

    fn process_use_stmt(&mut self, use_item: &'ast syn::ItemUse) {
        // Move visit_item_use logic here
        // Create an import node
        let import_id = self.state.next_node_id();

        // Process the use path
        let mut path_segments = Vec::new();
        let current_path = &use_item.tree;

        // Extract path segments from the use tree
        CodeVisitor::extract_use_path(current_path, &mut path_segments);

        // Create relations for the used types
        if !path_segments.is_empty() {
            // Create a synthetic type for the imported item
            let type_id = self.state.next_type_id();
            self.state.code_graph.type_graph.push(TypeNode {
                id: type_id,
                kind: TypeKind::Named {
                    path: path_segments.clone(),
                    is_fully_qualified: false,
                },
                related_types: Vec::new(),
            });

            // Add a Uses relation
            self.state.code_graph.relations.push(Relation {
                source: import_id,
                target: type_id,
                kind: RelationKind::Uses,
            });
        }

        // Continue visiting
        visit::visit_item_use(self, use_item);
        self.visit_item_use(use_item);
    }

    // moved into visit_item_use above
    // fn visit_item_use(&mut self, use_item: &'ast syn::ItemUse) {}

    fn process_extern_crate(&mut self, extern_crate: &'ast syn::ItemExternCrate) {
        // Moves visit_item_extern_crate logic here
        // Create an import node for extern crate
        let import_id = self.state.next_node_id();

        // Get the crate name
        let crate_name = extern_crate.ident.to_string();

        // Create a synthetic type for the extern crate
        let type_id = self.state.next_type_id();
        self.state.code_graph.type_graph.push(TypeNode {
            id: type_id,
            kind: TypeKind::Named {
                path: vec![crate_name.clone()],
                is_fully_qualified: false,
            },
            related_types: Vec::new(),
        });

        // Add a Uses relation
        self.state.code_graph.relations.push(Relation {
            source: import_id,
            target: type_id,
            kind: RelationKind::Uses,
        });

        // Continue visiting
        visit::visit_item_extern_crate(self, extern_crate);
    }

    // Add move other containment logic here as needed.
    // The below if placeholder, just copied and pasted from old
    // implementation, which started with:
    // impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    // Visit extern crate statements

    // Moved logic for this function into process_extern_crate above
    // fn visit_item_extern_crate(&mut self, extern_crate: &'ast syn::ItemExternCrate) {
}
