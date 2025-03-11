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

        // Process function parameters and track type relations
        let mut parameters = Vec::new();
        let mut param_type_ids = Vec::new();
        
        for arg in &func.sig.inputs {
            if let Some(param) = self.state.process_fn_arg(arg) {
                // Track parameter type relationship
                if let Some(type_id) = param.type_id {
                    self.state.code_graph.relations.push(Relation {
                        source: fn_id,
                        target: type_id,
                        kind: RelationKind::FunctionParameter,
                    });
                    param_type_ids.push(type_id);
                }
                parameters.push(param);
            }
        }

        // Extract return type if it exists and track relation
        let return_type = match &func.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => {
                let type_id = self.state.get_or_create_type(ty);
                // Add return type relationship
                self.state.code_graph.relations.push(Relation {
                    source: fn_id,
                    target: type_id,
                    kind: RelationKind::FunctionReturn,
                });
                Some(type_id)
            }
        };
        
        // Track generic parameter relationships
        for generic_param in &generic_params {
            if let GenericParamKind::Type { name, .. } = &generic_param.kind {
                let type_id = self.state.get_or_create_type(&syn::parse_str::<syn::Type>(name).unwrap());
                self.state.code_graph.relations.push(Relation {
                    source: fn_id,
                    target: type_id,
                    kind: RelationKind::GenericParameter,
                });
            }
        }

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
use super::state::VisitorState;
use super::utils::attributes::extract_attributes;
use super::utils::docs::extract_docstring;
use syn::{ItemFn, Signature, Pat, Type, FnArg, ReturnType};

pub trait FunctionVisitor {
    fn process_function(&mut self, func: &ItemFn);
}

impl<'ast> FunctionVisitor for VisitorState<'ast> {
    fn process_function(&mut self, func: &ItemFn) {
        let func_id = self.next_node_id();
        let func_name = func.sig.ident.to_string();
        let visibility = self.convert_visibility(&func.vis);
        let attributes = extract_attributes(&func.attrs);
        let docstring = extract_docstring(&func.attrs);

        let mut parameters = Vec::new();
        let mut return_type = None;

        for arg in &func.sig.inputs {
            match arg {
                FnArg::Typed(PatType { pat, ty, .. }) => {
                    let param_id = self.next_node_id();
                    let param_name = pat.to_token_stream().to_string();
                    let param_type_id = self.get_or_create_type(ty);
                    parameters.push(FunctionNode {
                        id: param_id,
                        name: param_name,
                        visibility: VisibilityKind::Inherited,
                        parameters: Vec::new(),
                        return_type: Some(param_type_id),
                        generic_params: Vec::new(),
                        attributes: Vec::new(),
                        docstring: None,
                        body: None,
                    });
                }
                _ => {}
            }
        }

        if let ReturnType::Type(_, ty) = &func.sig.output {
            return_type = Some(self.get_or_create_type(ty));
        }

        self.code_graph.functions.push(FunctionNode {
            id: func_id,
            name: func_name,
            visibility,
            parameters,
            return_type,
            generic_params: Vec::new(),
            attributes,
            docstring,
            body: Some(func.block.to_token_stream().to_string()),
        });
    }
}
