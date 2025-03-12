use crate::parser::nodes::{MacroKind, MacroNode, MacroRuleNode, NodeId, ProcMacroKind, VisibilityKind};
use crate::parser::relations::{Relation, RelationKind};
use crate::parser::visitor::processor::{AttributeOperations, CodeProcessor, DocOperations, StateManagement};
use crate::parser::visitor::type_processing::TypeProcessor;
use quote::ToTokens;
use syn::{Attribute, Item, ItemFn, ItemMacro, Visibility};

/// Trait for processing macro-related AST nodes
///
/// Builds on top of TypeProcessor for type resolution capabilities
pub trait MacroProcessor: TypeProcessor {
    /// Process a declarative macro definition (macro_rules!)
    fn process_declarative_macro(&mut self, macro_item: &ItemMacro) {
        // Only process macros with #[macro_export]
        if !macro_item.attrs.iter().any(|attr| attr.path().is_ident("macro_export")) {
            return;
        }

        let macro_id = self.next_node_id();
        
        // Get the macro name
        let macro_name = macro_item
            .ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or_else(|| "unnamed_macro".to_string());

        // Extract the macro body
        let body = Some(macro_item.mac.tokens.to_string());

        // Extract doc comments and other attributes
        let docstring = self.extract_docstring(&macro_item.attrs);
        let attributes = self.extract_attributes(&macro_item.attrs);

        // Parse macro rules 
        let rules = self.parse_macro_rules(&macro_item.mac.tokens.to_string());

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
        self.state_mut().code_graph.macros.push(macro_node);
    }

    /// Process a procedural macro function
    fn process_proc_macro(&mut self, func: &ItemFn) -> bool {
        // Check if this function is a procedural macro
        let is_proc_macro = func.attrs.iter().any(|attr| {
            attr.path().is_ident("proc_macro")
                || attr.path().is_ident("proc_macro_derive")
                || attr.path().is_ident("proc_macro_attribute")
        });

        if !is_proc_macro {
            return false;
        }

        let macro_id = self.next_node_id();
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
        let docstring = self.extract_docstring(&func.attrs);
        let attributes = self.extract_attributes(&func.attrs);

        // Extract function body as a string
        let body = Some(func.block.to_token_stream().to_string());

        // Create the macro node
        let macro_node = MacroNode {
            id: macro_id,
            name: macro_name,
            visibility: self.convert_visibility(&func.vis),
            kind: MacroKind::ProcedureMacro {
                kind: proc_macro_kind,
            },
            rules: Vec::new(), // Procedural macros don't have declarative rules
            attributes,
            docstring,
            body,
            expansion: None,
            parent_function: None,
        };

        // Add the macro to the code graph
        self.state_mut().code_graph.macros.push(macro_node);
        
        true
    }

    /// Process a macro invocation
    fn process_macro_invocation(&mut self, mac: &syn::Macro) {
        // Create a node ID for this macro invocation
        let invocation_id = self.next_node_id();

        // Get the macro name
        let macro_path = mac.path.to_token_stream().to_string();

        // Find if this macro is defined in our code graph
        let defined_macro = self
            .state_mut()
            .code_graph
            .macros
            .iter()
            .find(|m| m.name == macro_path.split("::").last().unwrap_or(&macro_path))
            .map(|m| m.id);

        if let Some(defined_macro_id) = defined_macro {
            // Add a relation between the invocation and the macro definition
            self.state_mut().code_graph.relations.push(Relation {
                source: invocation_id,
                target: defined_macro_id,
                kind: RelationKind::MacroUse,
            });
        }
    }

    /// Parse macro rules from token stream
    fn parse_macro_rules(&mut self, tokens_str: &str) -> Vec<MacroRuleNode> {
        let mut rules = Vec::new();
        
        // Very basic parsing of macro rules
        for rule in tokens_str.split(';') {
            if rule.trim().is_empty() {
                continue;
            }

            // Try to split the rule into pattern and expansion
            if let Some(idx) = rule.find("=>") {
                let pattern = rule[..idx].trim().to_string();
                let expansion = rule[(idx + 2)..].trim().to_string();

                rules.push(MacroRuleNode {
                    id: self.next_node_id(),
                    pattern,
                    expansion,
                });
            }
        }
        
        rules
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
impl<T> MacroProcessor for T 
where 
    T: TypeProcessor
{}
