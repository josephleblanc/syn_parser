use crate::common::*;
use syn_parser::parser::nodes::*;
use syn_parser::parser::types::*;

#[test]
fn test_module_parsing() {
    let graph = parse_fixture("modules.rs");

    // Check the root module
    let root_module = graph.modules.iter().find(|m| m.name == "root").expect("Root module not found");
    assert_eq!(root_module.name, "root");
    assert_eq!(root_module.visibility, VisibilityKind::Inherited);
    assert_eq!(root_module.submodules.len(), 1);

    // Check the inner module
    let inner_module_id = root_module.submodules[0];
    let inner_module = graph.modules.iter().find(|m| m.id == inner_module_id).expect("Inner module not found");
    assert_eq!(inner_module.name, "inner_module");
    assert_eq!(inner_module.visibility, VisibilityKind::Inherited);
    assert_eq!(inner_module.submodules.len(), 0);
    assert_eq!(inner_module.items.len(), 1);

    // Check the inner function
    let inner_function_id = inner_module.items[0];
    let inner_function = graph.functions.iter().find(|f| f.id == inner_function_id).expect("Inner function not found");
    assert_eq!(inner_function.name, "inner_function");
    assert_eq!(inner_function.visibility, VisibilityKind::Public);
    assert_eq!(inner_function.parameters.len(), 0);
    assert_eq!(inner_function.return_type, None);

    // Check the outer function
    let outer_function = graph.functions.iter().find(|f| f.name == "outer_function").expect("Outer function not found");
    assert_eq!(outer_function.name, "outer_function");
    assert_eq!(outer_function.visibility, VisibilityKind::Public);
    assert_eq!(outer_function.parameters.len(), 0);
    assert_eq!(outer_function.return_type, None);
}
