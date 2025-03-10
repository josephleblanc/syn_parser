use crate::common::*;
use syn_parser::parser::nodes::*;
use syn_parser::parser::types::*;

#[test]
fn test_module_parsing() {
    let graph = parse_fixture("modules.rs");

    // Print all modules for debugging
    for module in &graph.modules {
        println!("Module: {}, Visibility: {:?}", module.name, module.visibility);
        println!("Submodules: {:?}", module.submodules);
        println!("Items: {:?}", module.items);
    }

    // Check the root module
    let root_module = graph.modules.iter().find(|m| m.name == "root").expect("Root module not found");
    assert_eq!(root_module.name, "root", "Root module name should be 'root', but it is {:?}", root_module.name);
    assert_eq!(root_module.visibility, VisibilityKind::Inherited, "Root module visibility should be Inherited, but it is {:?}", root_module.visibility);
    assert_eq!(root_module.submodules.len(), 1, "Root module should have exactly one submodule, but it has {:?}", root_module.submodules.len());

    // Check the inner module
    if let Some(&inner_module_id) = root_module.submodules.get(0) {
        let inner_module = graph.modules.iter().find(|m| m.id == inner_module_id).expect("Inner module not found");
        assert_eq!(inner_module.name, "inner_module", "Inner module name should be 'inner_module', but it is {:?}", inner_module.name);
        assert_eq!(inner_module.visibility, VisibilityKind::Inherited, "Inner module visibility should be Inherited, but it is {:?}", inner_module.visibility);
        assert_eq!(inner_module.submodules.len(), 0, "Inner module should have no submodules, but it has {:?}", inner_module.submodules.len());
        assert_eq!(inner_module.items.len(), 1, "Inner module should have exactly one item, but it has {:?}", inner_module.items.len());

        // Check the inner function
        if let Some(&inner_function_id) = inner_module.items.get(0) {
            let inner_function = graph.functions.iter().find(|f| f.id == inner_function_id).expect("Inner function not found");
            assert_eq!(inner_function.name, "inner_function", "Inner function name should be 'inner_function', but it is {:?}", inner_function.name);
            assert_eq!(inner_function.visibility, VisibilityKind::Public, "Inner function visibility should be Public, but it is {:?}", inner_function.visibility);
            assert_eq!(inner_function.parameters.len(), 0, "Inner function should have no parameters, but it has {:?}", inner_function.parameters.len());
            assert_eq!(inner_function.return_type, None, "Inner function should have no return type, but it has {:?}", inner_function.return_type);
        } else {
            panic!("Inner module should have at least one item, but it has none");
        }
    } else {
        panic!("Root module should have at least one submodule, but it has none");
    }

    // Check the outer function
    let outer_function = graph.functions.iter().find(|f| f.name == "outer_function").expect("Outer function not found");
    assert_eq!(outer_function.name, "outer_function", "Outer function name should be 'outer_function', but it is {:?}", outer_function.name);
    assert_eq!(outer_function.visibility, VisibilityKind::Public, "Outer function visibility should be Public, but it is {:?}", outer_function.visibility);
    assert_eq!(outer_function.parameters.len(), 0, "Outer function should have no parameters, but it has {:?}", outer_function.parameters.len());
    assert_eq!(outer_function.return_type, None, "Outer function should have no return type, but it has {:?}", outer_function.return_type);
}
