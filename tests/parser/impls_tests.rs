use crate::common::*;
use syn_parser::parser::nodes::*;
use syn_parser::parser::types::*;

#[test]
fn test_impl_for_struct() {
    let graph = parse_fixture("impls.rs");

    let impl_node = find_impl_for_type(&graph, "SampleStruct").expect("Impl for SampleStruct not found");

    assert_eq!(impl_node.methods.len(), 2);
    assert_eq!(impl_node.methods[0].name, "new");
    assert_eq!(impl_node.methods[1].name, "get_field");
}

#[test]
fn test_impl_for_trait() {
    let graph = parse_fixture("impls.rs");

    let impl_node = find_impl_for_type(&graph, "SampleStruct").expect("Impl for SampleStruct not found");

    assert_eq!(impl_node.methods.len(), 2);
    assert_eq!(impl_node.methods[0].name, "new");
    assert_eq!(impl_node.methods[1].name, "get_field");

    if let Some(trait_type_id) = impl_node.trait_type {
        let trait_node = graph.traits.iter().find(|t| t.id == trait_type_id).expect("Trait not found");
        assert_eq!(trait_node.name, "SampleTrait");
        assert_eq!(trait_node.methods.len(), 1);
        assert_eq!(trait_node.methods[0].name, "sample_method");
    } else {
        panic!("Trait type not found in impl block");
    }
}

#[test]
fn test_generic_impl_for_struct() {
    let graph = parse_fixture("impls.rs");

    let impl_node = find_impl_for_type(&graph, "GenericStruct").expect("Impl for GenericStruct not found");

    assert_eq!(impl_node.methods.len(), 2);
    assert_eq!(impl_node.methods[0].name, "new");
    assert_eq!(impl_node.methods[1].name, "get_field");

    assert_eq!(impl_node.generic_params.len(), 1);
    if let GenericParamKind::Type { name, .. } = &impl_node.generic_params[0].kind {
        assert_eq!(name, "T");
    } else {
        panic!("Expected Type generic parameter");
    }
}

#[test]
fn test_generic_impl_for_trait() {
    let graph = parse_fixture("impls.rs");

    let impl_node = find_impl_for_type(&graph, "GenericStruct").expect("Impl for GenericStruct not found");

    assert_eq!(impl_node.methods.len(), 2);
    assert_eq!(impl_node.methods[0].name, "new");
    assert_eq!(impl_node.methods[1].name, "get_field");

    if let Some(trait_type_id) = impl_node.trait_type {
        let trait_node = graph.traits.iter().find(|t| t.id == trait_type_id).expect("Trait not found");
        assert_eq!(trait_node.name, "GenericTrait");
        assert_eq!(trait_node.methods.len(), 1);
        assert_eq!(trait_node.methods[0].name, "generic_method");
        assert_eq!(trait_node.generic_params.len(), 1);
        if let GenericParamKind::Type { name, .. } = &trait_node.generic_params[0].kind {
            assert_eq!(name, "T");
        } else {
            panic!("Expected Type generic parameter");
        }
    } else {
        panic!("Trait type not found in impl block");
    }
}
