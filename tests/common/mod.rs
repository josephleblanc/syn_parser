use std::path::Path;
use crate::parser::{analyze_code, CodeGraph, nodes::*, types::*};

pub fn parse_fixture(fixture_name: &str) -> CodeGraph {
    let path = Path::new("tests/fixtures").join(fixture_name);
    analyze_code(&path).expect("Failed to parse fixture")
}

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

pub fn find_trait_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a TraitNode> {
    graph.traits.iter().find(|t| t.name == name)
}

pub fn find_function_by_name<'a>(graph: &'a CodeGraph, name: &str) -> Option<&'a FunctionNode> {
    graph.functions.iter().find(|f| f.name == name)
}
