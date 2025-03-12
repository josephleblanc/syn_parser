use crate::common::*;
use syn_parser::parser::types::GenericParamKind;
use syn_parser::parser::{
    nodes::FunctionNode, types::*, visitor::utils::generics::process_generics,
};

#[test]
fn test_impl_for_struct() {
    let graph = parse_fixture("impls.rs");

    // Find the impl block for SampleStruct that is not a trait impl
    let impl_node = graph
        .impls
        .iter()
        .find(|impl_node| {
            if impl_node.trait_type.is_none() {
                if let Some(type_node) = graph
                    .type_graph
                    .iter()
                    .find(|t| t.id == impl_node.self_type)
                {
                    if let TypeKind::Named { path, .. } = &type_node.kind {
                        return path.last().map_or(false, |s| s == "SampleStruct");
                    }
                }
            }
            false
        })
        .expect("Impl for SampleStruct not found");

    assert_eq!(impl_node.methods.len(), 2);
    assert_eq!(impl_node.methods[0].name, "new");
    assert_eq!(impl_node.methods[1].name, "get_field");
}

#[test]
fn test_impl_for_trait() {
    let graph = parse_fixture("impls.rs");

    // Find the impl block for SampleStruct that implements SampleTrait
    let impl_node = graph
        .impls
        .iter()
        .find(|impl_node| {
            if let Some(trait_type_id) = impl_node.trait_type {
                if let Some(type_node) = graph.type_graph.iter().find(|t| t.id == trait_type_id) {
                    if let TypeKind::Named { path, .. } = &type_node.kind {
                        if path.last().map_or(false, |s| s == "SampleTrait") {
                            if let Some(self_type) = graph
                                .type_graph
                                .iter()
                                .find(|t| t.id == impl_node.self_type)
                            {
                                if let TypeKind::Named {
                                    path: self_path, ..
                                } = &self_type.kind
                                {
                                    return self_path.last().map_or(false, |s| s == "SampleStruct");
                                }
                            }
                        }
                    }
                }
            }
            false
        })
        .expect("Impl of SampleTrait for SampleStruct not found");

    assert_eq!(impl_node.methods.len(), 1);
    assert_eq!(impl_node.methods[0].name, "sample_method");

    let _trait_type_id = impl_node
        .trait_type
        .expect("Trait type not found in impl block");
    let trait_node = find_trait_by_name(&graph, "SampleTrait").expect("Trait not found");
    assert_eq!(trait_node.name, "SampleTrait");
    assert_eq!(trait_node.methods.len(), 1);
    assert_eq!(trait_node.methods[0].name, "sample_method");
}

#[test]
fn test_find_impl_by_name() {
    let graph = parse_fixture("impls.rs");

    // Find the impl block for SampleTrait directly
    let sample_trait_impl = graph
        .impls
        .iter()
        .find(|impl_node| {
            if let Some(trait_type_id) = impl_node.trait_type {
                if let Some(type_node) = graph.type_graph.iter().find(|t| t.id == trait_type_id) {
                    if let TypeKind::Named { path, .. } = &type_node.kind {
                        return path.last().map_or(false, |s| s == "SampleTrait");
                    }
                }
            }
            false
        })
        .expect("Impl for SampleTrait not found");

    assert_eq!(sample_trait_impl.methods.len(), 1);
    assert_eq!(sample_trait_impl.methods[0].name, "sample_method");

    // Find the impl block for GenericTrait directly
    let generic_trait_impl = graph
        .impls
        .iter()
        .find(|impl_node| {
            if let Some(trait_type_id) = impl_node.trait_type {
                if let Some(type_node) = graph.type_graph.iter().find(|t| t.id == trait_type_id) {
                    if let TypeKind::Named { path, .. } = &type_node.kind {
                        return path.last().map_or(false, |s| s == "GenericTrait");
                    }
                }
            }
            false
        })
        .expect("Impl for GenericTrait not found");

    assert_eq!(generic_trait_impl.methods.len(), 1);
    assert_eq!(generic_trait_impl.methods[0].name, "generic_method");
}

#[test]
fn test_generic_impl_for_struct() {
    let graph = parse_fixture("impls.rs");

    // Find the impl block for GenericStruct that is not a trait impl
    let impl_node = graph
        .impls
        .iter()
        .find(|impl_node| {
            if impl_node.trait_type.is_none() {
                if let Some(type_node) = graph
                    .type_graph
                    .iter()
                    .find(|t| t.id == impl_node.self_type)
                {
                    if let TypeKind::Named { path, .. } = &type_node.kind {
                        return path.last().map_or(false, |s| s == "GenericStruct");
                    }
                }
            }
            false
        })
        .expect("Impl for GenericStruct not found");

    assert_eq!(impl_node.methods.len(), 2);
    assert_eq!(impl_node.methods[0].name, "new");
    assert_eq!(impl_node.methods[1].name, "get_field");

    assert_eq!(impl_node.generic_params.len(), 1);
    if let GenericParamKind::Type { name, .. } = &impl_node.generic_params[0].kind {
        assert_eq!(name, "T");
    } else {
        panic!("Expected Type generic parameter");
    }

    assert!(impl_node.generic_params.iter().any(|param| {
        matches!(&param.kind,
            GenericParamKind::Const { name, .. } if name == "SIZE"
        )
    }));
}

#[test]
fn test_generic_impl_for_trait() {
    let graph = parse_fixture("impls.rs");

    // Find the impl block for GenericStruct that implements GenericTrait
    let impl_node = graph
        .impls
        .iter()
        .find(|impl_node| {
            if let Some(trait_type_id) = impl_node.trait_type {
                if let Some(type_node) = graph.type_graph.iter().find(|t| t.id == trait_type_id) {
                    if let TypeKind::Named { path, .. } = &type_node.kind {
                        if path.last().map_or(false, |s| s == "GenericTrait") {
                            if let Some(self_type) = graph
                                .type_graph
                                .iter()
                                .find(|t| t.id == impl_node.self_type)
                            {
                                if let TypeKind::Named {
                                    path: self_path, ..
                                } = &self_type.kind
                                {
                                    return self_path
                                        .last()
                                        .map_or(false, |s| s == "GenericStruct");
                                }
                            }
                        }
                    }
                }
            }
            false
        })
        .expect("Impl of GenericTrait for GenericStruct not found");

    assert_eq!(impl_node.methods.len(), 1);
    assert_eq!(impl_node.methods[0].name, "generic_method");

    let _trait_type_id = impl_node
        .trait_type
        .expect("Trait type not found in impl block");
    let trait_node = find_trait_by_name(&graph, "GenericTrait").expect("Trait not found");
    assert_eq!(trait_node.name, "GenericTrait");
    assert_eq!(trait_node.methods.len(), 1);
    assert_eq!(trait_node.methods[0].name, "generic_method");
    assert_eq!(trait_node.generic_params.len(), 1);
    if let GenericParamKind::Type { name, .. } = &trait_node.generic_params[0].kind {
        assert_eq!(name, "T");
    } else {
        panic!("Expected Type generic parameter");
    }
}
