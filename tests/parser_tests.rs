use std::path::PathBuf;
use syn_parser::parser::{analyze_code, save_graph, RelationKind, TypeDefNode, VisibilityKind};
mod data;

#[test]
fn test_analyzer() {
    let input_path = PathBuf::from("tests/data/sample.rs");
    let output_path = PathBuf::from("tests/data/code_graph.ron");

    let code_graph_result = analyze_code(&input_path);
    assert!(code_graph_result.is_ok());

    let code_graph = code_graph_result.unwrap();
    save_graph(&code_graph, &output_path).expect("Failed to save graph");

    // Check the number of functions,
    // tree-sitter name: function_item
    assert!(
        !code_graph.functions.is_empty(),
        "No functions found in the code graph"
    );
    assert_eq!(
        code_graph.functions.len(),
        1,
        "Expected 1 function in the code graph"
    );

    // Check the number of defined types (structs, enums)
    // tree-sitter name: struct_item, enum_item
    assert!(
        !code_graph.defined_types.is_empty(),
        "No defined types found in the code graph"
    );
    assert_eq!(
        code_graph.defined_types.len(),
        3,
        "Expected 3 defined types in the code graph"
    );

    // Check the number of traits
    // tree-sitter name: trait_item
    assert!(
        !code_graph.traits.is_empty(),
        "No traits found in the code graph"
    );
    assert_eq!(
        code_graph.traits.len(),
        2,
        "Expected 2 traits in the code graph"
    );

    // Check the number of impls
    // tree-sitter name: impl_item
    assert!(
        !code_graph.impls.is_empty(),
        "No impls found in the code graph"
    );
    assert_eq!(
        code_graph.impls.len(),
        4,
        "Expected 4 impl in the code graph"
    );

    // Check the number of modules
    // tree-sittern name: mod_item

    // Check for number of relations
    // should be correct number in sample.rs

    // Check specific function details

    // Check specific defined type details
    let defined_type = &code_graph.defined_types[0];
    match defined_type {
        TypeDefNode::Struct(struct_node) => {
            assert_eq!(struct_node.name, "SampleStruct", "Struct name mismatch");
            assert_eq!(
                struct_node.fields.len(),
                1,
                "Expected 1 fields for struct SampleStruct"
            );
            assert_eq!(
                struct_node.generic_params.len(),
                1,
                "Expected 1 generic parameter for struct SampleStruct"
            );
            assert_eq!(
                struct_node.attributes.len(),
                1,
                "Expected 1 attributes for struct SampleStruct"
            );
            assert_eq!(
                struct_node.docstring, None,
                "Expected no docstring for struct SampleStruct"
            );
        }
        _ => panic!("Expected a struct, found a different type"),
    }

    // Check specific trait details
    // TODO: add tests

    // Check specific impl details
    let impl_node = &code_graph.impls[0];
    assert_eq!(impl_node.methods.len(), 1, "Expected 1 method for impl");
    assert_eq!(
        impl_node.generic_params.len(),
        1,
        "Expected 1 generic parameter for impl"
    );
    assert_eq!(
        impl_node.trait_type,
        Some(code_graph.traits[0].id),
        "Expected trait type to match trait id"
    );

    // Check specific module details

    // Check specific relation details

    // Check specific enum details
    let enum_node = &code_graph
        .defined_types
        .iter()
        .find(|def| matches!(def, TypeDefNode::Enum(_)))
        .expect("Expected an enum");
    if let TypeDefNode::Enum(enum_) = enum_node {
        assert_eq!(enum_.name, "SampleEnum", "Enum name mismatch");
        assert_eq!(
            enum_.variants.len(),
            2,
            "Expected 2 variants for enum SampleEnum"
        );
        assert_eq!(
            enum_.generic_params.len(),
            1,
            "Expected 1 generic parameter for enum SampleEnum"
        );
        assert_eq!(
            enum_.attributes.len(),
            1,
            "Expected 1 attributes for enum SampleEnum"
        );
        assert_eq!(
            enum_.docstring, None,
            "Expected no docstring for enum SampleEnum"
        );

        // Check variant details
        let variant1 = &enum_.variants[0];
        assert_eq!(variant1.name, "Variant1", "Variant name mismatch");
        assert_eq!(
            variant1.fields.len(),
            0,
            "Expected 0 fields for variant Variant1"
        );
        assert_eq!(
            variant1.discriminant, None,
            "Expected no discriminant for variant Variant1"
        );
        assert_eq!(
            variant1.attributes.len(),
            0,
            "Expected 0 attributes for variant Variant1"
        );

        let variant2 = &enum_.variants[1];
        assert_eq!(variant2.name, "Variant2", "Variant name mismatch");
        assert_eq!(
            variant2.fields.len(),
            1,
            "Expected 1 field for variant Variant2"
        );
        assert_eq!(
            variant2.discriminant, None,
            "Expected no discriminant for variant Variant2"
        );
        assert_eq!(
            variant2.attributes.len(),
            0,
            "Expected 0 attributes for variant Variant2"
        );

        let field1 = &variant2.fields[0];
        assert_eq!(
            field1.name, None,
            "Expected no name for field in variant Variant2"
        );
        assert_eq!(
            field1.visibility,
            VisibilityKind::Public,
            "Expected public visibility for field in variant Variant2"
        );
        assert_eq!(
            field1.attributes.len(),
            0,
            "Expected 0 attributes for field in variant Variant2"
        );
    } else {
        panic!("Expected an enum, found a different type");
    }

    // Check specific struct details
    let struct_node = &code_graph
        .defined_types
        .iter()
        .find(|def| matches!(def, TypeDefNode::Struct(_)))
        .expect("Expected a struct");
    if let TypeDefNode::Struct(struct_) = struct_node {
        assert_eq!(struct_.name, "NestedStruct", "Struct name mismatch");
        assert_eq!(
            struct_.fields.len(),
            1,
            "Expected 1 field for struct NestedStruct"
        );
        assert_eq!(
            struct_.generic_params.len(),
            0,
            "Expected 0 generic parameters for struct NestedStruct"
        );
        assert_eq!(
            struct_.attributes.len(),
            0,
            "Expected 0 attributes for struct NestedStruct"
        );
        assert_eq!(
            struct_.docstring, None,
            "Expected no docstring for struct NestedStruct"
        );

        // Check field details
        let field1 = &struct_.fields[0];
        assert_eq!(
            field1.name,
            Some("nested_field".to_string()),
            "Expected field name nested_field for struct NestedStruct"
        );
        assert_eq!(
            field1.visibility,
            VisibilityKind::Public,
            "Expected public visibility for field nested_field in struct NestedStruct"
        );
        assert_eq!(
            field1.attributes.len(),
            0,
            "Expected 0 attributes for field nested_field in struct NestedStruct"
        );
    } else {
        panic!("Expected a struct, found a different type");
    }

    println!("Code graph saved to {:?}", output_path);
}
