// functions.rs
use syn::{visit, ItemFn, FnArg, ReturnType};
use crate::parser::{
    nodes::{FunctionNode, ParameterNode, VisibilityKind, MacroNode, MacroKind, ProcMacroKind},
    relations::{Relation, RelationKind},
    types::TypeId,
    visitor::{state::VisitorState, utils::generics::process_generics}
};

pub trait FunctionVisitor<'ast> {
    fn process_function(&mut self, func: &'ast ItemFn, state: &mut VisitorState);
}

impl<'ast> FunctionVisitor<'ast> for super::CodeVisitor<'ast> {
    fn process_function(&mut self, func: &'ast ItemFn, state: &mut VisitorState) {
        // Check if this function is a procedural macro
        let is_proc_macro = func.attrs.iter().any(|attr| {
            attr.path().is_ident("proc_macro")
                || attr.path().is_ident("proc_macro_derive")
                || attr.path().is_ident("proc_macro_attribute")
        });

        if is_proc_macro {
            let macro_id = self.state.next_node_id();
            let macro_name = func.sig.ident.to_string();

            // Determine the kind of procedural macro
            let proc_macro_kind = if func
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("proc_macro_derive"))
            {
                ProcMacroKind::Derive
            } else if func
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("proc_macro_attribute"))
            {
                ProcMacroKind::Attribute
            } else {
                ProcMacroKind::Function
            };

            // Extract doc comments and other attributes
            let docstring = self.state.extract_docstring(&func.attrs);
            let attributes = self.state.extract_attributes(&func.attrs);

            // Extract function body as a string
            let body = Some(func.block.to_token_stream().to_string());

            // Create the macro node
            let macro_node = MacroNode {
                id: macro_id,
                name: macro_name,
                visibility: self.state.convert_visibility(&func.vis),
                kind: MacroKind::ProcedureMacro {
                    kind: proc_macro_kind,
                },
                rules: Vec::new(), // Procedural macros don't have declarative rules
                attributes,
                docstring,
                body,
            };

            // Add the macro to the code graph
            self.state.code_graph.macros.push(macro_node);
        }

        let fn_id = self.state.next_node_id();
        let fn_name = func.sig.ident.to_string();

        // Process function parameters
        let mut parameters = Vec::new();
        for arg in &func.sig.inputs {
            if let Some(param) = self.state.process_fn_arg(arg) {
                // Add relation between function and parameter
                self.state.code_graph.relations.push(Relation {
                    source: fn_id,
                    target: param.id,
                    kind: RelationKind::FunctionParameter,
                });
                parameters.push(param);
            }
        }

        // Extract return type if it exists
        let return_type = match &func.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => {
                let type_id = self.state.get_or_create_type(ty);
                // Add relation between function and return type
                self.state.code_graph.relations.push(Relation {
                    source: fn_id,
                    target: type_id,
                    kind: RelationKind::FunctionReturn,
                });
                Some(type_id)
            }
        };

        // Process generic parameters
        let generic_params = self.state.process_generics(&func.sig.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&func.attrs);
        let attributes = self.state.extract_attributes(&func.attrs);

        // Extract function body as a string
        let body = Some(func.block.to_token_stream().to_string());

        // Store function info
        self.state.code_graph.functions.push(FunctionNode {
            id: fn_id,
            name: fn_name,
            visibility: self.state.convert_visibility(&func.vis),
            parameters,
            return_type,
            generic_params,
            attributes,
            docstring,
            body,
        });

        // Continue visiting the function body
        visit::visit_item_fn(self, func);
    }
}
