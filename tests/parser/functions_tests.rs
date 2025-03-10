use common::*;
use parser::nodes::*;
use parser::types::*;

#[test]
fn test_regular_function_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "regular_function")
        .expect("regular_function not found");

    assert_eq!(function.name, "regular_function");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert!(function.parameters.is_empty());
    assert_eq!(function.return_type, None);
    assert!(function.generic_params.is_empty());
    assert_eq!(function.attributes, vec![]);
    assert_eq!(function.docstring, None);
}

#[test]
fn test_function_with_params_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "function_with_params")
        .expect("function_with_params not found");

    assert_eq!(function.name, "function_with_params");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert_eq!(function.parameters.len(), 2);
    assert_eq!(function.parameters[0].name, Some("x".to_string()));
    assert_eq!(function.parameters[0].type_id, TypeId::from(0)); // Adjust type_id as needed
    assert_eq!(function.parameters[1].name, Some("y".to_string()));
    assert_eq!(function.parameters[1].type_id, TypeId::from(0)); // Adjust type_id as needed
    assert_eq!(function.return_type, Some(TypeId::from(0))); // Adjust type_id as needed
    assert!(function.generic_params.is_empty());
    assert_eq!(function.attributes, vec![]);
    assert_eq!(function.docstring, None);
}

#[test]
fn test_generic_function_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "generic_function")
        .expect("generic_function not found");

    assert_eq!(function.name, "generic_function");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert_eq!(function.parameters.len(), 1);
    assert_eq!(function.parameters[0].name, Some("arg".to_string()));
    assert_eq!(function.parameters[0].type_id, TypeId::from(0)); // Adjust type_id as needed
    assert_eq!(function.return_type, Some(TypeId::from(0))); // Adjust type_id as needed
    assert_eq!(function.generic_params.len(), 1);
    assert_eq!(function.generic_params[0].kind, GenericParamKind::Type { name: "T".to_string(), default: None });
    assert_eq!(function.attributes, vec![]);
    assert_eq!(function.docstring, None);
}

#[test]
fn test_attributed_function_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "attributed_function")
        .expect("attributed_function not found");

    assert_eq!(function.name, "attributed_function");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert!(function.parameters.is_empty());
    assert_eq!(function.return_type, None);
    assert!(function.generic_params.is_empty());
    assert_eq!(function.attributes.len(), 1);
    assert_eq!(function.attributes[0].name, "cfg");
    assert_eq!(function.attributes[0].args, vec!["test".to_string()]);
    assert_eq!(function.docstring, None);
}

#[test]
fn test_documented_function_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "documented_function")
        .expect("documented_function not found");

    assert_eq!(function.name, "documented_function");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert!(function.parameters.is_empty());
    assert_eq!(function.return_type, None);
    assert!(function.generic_params.is_empty());
    assert_eq!(function.attributes, vec![]);
    assert!(function.docstring.is_some());
    assert!(function.docstring.as_ref().unwrap().contains("documented function"));
}

#[test]
fn test_unsafe_function_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "unsafe_function")
        .expect("unsafe_function not found");

    assert_eq!(function.name, "unsafe_function");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert!(function.parameters.is_empty());
    assert_eq!(function.return_type, None);
    assert!(function.generic_params.is_empty());
    assert_eq!(function.attributes, vec![]);
    assert_eq!(function.docstring, None);
}

#[test]
fn test_lifetime_function_parsing() {
    let graph = parse_fixture("functions.rs");

    let function = find_function_by_name(&graph, "lifetime_function")
        .expect("lifetime_function not found");

    assert_eq!(function.name, "lifetime_function");
    assert_eq!(function.visibility, VisibilityKind::Inherited);
    assert_eq!(function.parameters.len(), 1);
    assert_eq!(function.parameters[0].name, Some("arg".to_string()));
    assert_eq!(function.parameters[0].type_id, TypeId::from(0)); // Adjust type_id as needed
    assert_eq!(function.return_type, Some(TypeId::from(0))); // Adjust type_id as needed
    assert!(function.generic_params.is_empty());
    assert_eq!(function.attributes, vec![]);
    assert_eq!(function.docstring, None);
}
