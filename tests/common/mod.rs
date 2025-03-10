use std::path::Path;
use syn_parser::parser::graph::CodeGraph;
use syn_parser::parser::nodes::*;
use syn_parser::parser::types::{GenericParamNode, GenericParamKind, TypeKind};
use syn_parser::parser::visitor::analyze_code;

/// Parse a fixture file and return the resulting CodeGraph
pub fn parse_fixture(fixture_name: &str) -> CodeGraph {
    let path = Path::new("tests/fixtures").join(fixture_name);
    analyze_code(&path).expect("Failed to parse fixture")
}

/// Find a struct by name in the code graph
pub fn find_struct_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a StructNode> {
    graph.defined_types.iter().find_map(|def| {
        if let TypeDefNode::Struct(s) = def {
            if s.name == name {
                return Some(s);
            }
        }
        None
    })
}

/// Find an enum by name in the code graph
pub fn find_enum_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a EnumNode> {
    graph.defined_types.iter().find_map(|def| {
        if let TypeDefNode::Enum(e) = def {
            if e.name == name {
                return Some(e);
            }
        }
        None
    })
}

/// Find a trait by name in the code graph
pub fn find_trait_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a TraitNode> {
    graph.traits.iter().find(|t| t.name == name).or_else(|| {
        graph.private_traits.iter().find(|t| t.name == name)
    })
}

/// Find a function by name in the code graph
pub fn find_function_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a FunctionNode> {
    graph.functions.iter().find(|f| f.name == name)
}

/// Find an impl block for a specific type
pub fn find_impl_for_type<'a>(graph: &'a CodeGraph, type_name: &str) -> Option<&'a ImplNode> {
    graph.impls.iter().find(|impl_node| {
        if let Some(type_node) = graph.type_graph.iter().find(|t| t.id == impl_node.self_type) {
            match &type_node.kind {
                TypeKind::Named { path, .. } => {
                    path.last().map(|s| s.as_str()) == Some(type_name)
                }
                _ => false,
            }
        } else {
            false
        }
    })
}

/// Find a module by name in the code graph
pub fn find_module_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a ModuleNode> {
    graph.modules.iter().find(|m| m.name == name)
}

pub fn find_generic_param_by_name<'a>(params: &'a [GenericParamNode], name: &str) -> Option<&'a GenericParamNode> {
    params.iter().find(|param| {
        match &param.kind {
            GenericParamKind::Lifetime { name: param_name, .. } => param_name == name,
            _ => false,
        }
    })
}

/// Find an impl block by trait name
pub fn find_impl_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a ImplNode> {
    graph.impls.iter().find(|impl_node| {
        if let Some(trait_type_id) = impl_node.trait_type {
            if let Some(trait_node) = graph.traits.iter().find(|t| t.id == trait_type_id) {
                return trait_node.name == name;
            }
        }
        false
    })
}
