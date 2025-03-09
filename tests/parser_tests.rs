use std::path::PathBuf;
use syn_parser::parser::*;
mod data;
#[test]
fn test_analyzer() {
    let input_path = PathBuf::from("tests/data/sample.rs");
    let output_path = PathBuf::from("tests/data/code_graph.ron");

    let code_graph_result = analyze_code(&input_path);
    assert!(code_graph_result.is_ok());

    let code_graph = code_graph_result.unwrap();
    save_graph(&code_graph, &output_path).expect("Failed to save graph");

    // =========== Entity Counts ===========
    // Check functions
    assert_eq!(
        code_graph.functions.len(),
        2,
        "Expected 2 functions in the code graph (sample_function, public_function_in_private_module)\nFound:\n\t{}\n\t{}",
        code_graph
            .functions
            .iter()
            .find(|f| f.name == "sample_function")
            .expect("sample_function not found").name,
        code_graph
            .functions
            .iter()
            .find(|f| f.name == "public_function_in_private_module")
            .expect("public_function_in_private_module not found").name
    );

    // Check defined types
    assert_eq!(
        code_graph.defined_types.len(),
        8,
        "Expected 8 defined types (SampleStruct, NestedStruct, SampleEnum, PrivateStruct, ModuleStruct, TupleStruct, UnitStruct, and one more)"
    );

    // Check traits
    assert_eq!(
        code_graph.traits.len(),
        3,
        "Expected 3 traits (SampleTrait, AnotherTrait, DefaultTrait)"
    );

    // Check impls
    assert_eq!(
        code_graph.impls.len(),
        6,
        "Expected 6 impls (SampleTrait for SampleStruct, AnotherTrait for SampleStruct, DefaultTrait for SampleStruct, SampleStruct direct, DefaultTrait for ModuleStruct, and PrivateStruct)"
    );

    // Check modules
    assert_eq!(
        code_graph.modules.len(),
        3,
        "Expected 3 modules (root, private_module, public_module)"
    );

    // =========== Relations ===========
    // Count relations by type
    let trait_impl_relations = code_graph
        .relations
        .iter()
        .filter(|r| r.kind == RelationKind::ImplementsTrait)
        .count();
    assert_eq!(trait_impl_relations, 4, "Expected 4 'implements' relations");

    let contains_relations = code_graph
        .relations
        .iter()
        .filter(|r| r.kind == RelationKind::Contains)
        .count();
    assert!(
        contains_relations > 0,
        "Expected 'contains' relations between modules and their contents"
    );

    let uses_type_relations = code_graph
        .relations
        .iter()
        .filter(|r| r.kind == RelationKind::Uses)
        .count();
    assert!(
        uses_type_relations > 0,
        "Expected 'uses type' relations for `use` statements"
    );

    // =========== Struct Tests ===========
    // Find SampleStruct by name
    let sample_struct = code_graph
        .defined_types
        .iter()
        .find(|def| match def {
            TypeDefNode::Struct(s) => s.name == "SampleStruct",
            _ => false,
        })
        .expect("SampleStruct not found");

    if let TypeDefNode::Struct(struct_node) = sample_struct {
        // Check basic properties
        assert_eq!(struct_node.name, "SampleStruct");
        assert_eq!(struct_node.visibility, VisibilityKind::Public);

        // Check fields
        assert_eq!(
            struct_node.fields.len(),
            1,
            "Expected 1 field in SampleStruct"
        );
        assert_eq!(struct_node.fields[0].name, Some("field".to_string()));
        assert_eq!(struct_node.fields[0].visibility, VisibilityKind::Public);

        // Check generics
        assert_eq!(
            struct_node.generic_params.len(),
            1,
            "Expected 1 generic parameter"
        );
        assert_eq!(
            if let GenericParamKind::Type { name, .. } = &struct_node.generic_params[0].kind {
                name
            } else {
                "Not a GenericParamKind::Type"
            },
            "T"
        );

        // Check attributes and docstring
        assert!(struct_node
            .attributes
            .iter()
            .any(|attr| attr.name == "derive"));
        assert!(
            struct_node.docstring.is_some(),
            "Expected docstring for SampleStruct"
        );
        assert!(struct_node
            .docstring
            .as_ref()
            .unwrap()
            .contains("sample struct with a generic parameter"));
    }

    // Check tuple struct
    let tuple_struct = code_graph
        .defined_types
        .iter()
        .find(|def| match def {
            TypeDefNode::Struct(s) => s.name == "TupleStruct",
            _ => false,
        })
        .expect("TupleStruct not found");

    if let TypeDefNode::Struct(struct_node) = tuple_struct {
        assert_eq!(
            struct_node.fields.len(),
            2,
            "Expected 2 fields in TupleStruct"
        );
        // Tuple struct fields typically don't have names in the parsed representation
        assert_eq!(struct_node.fields[0].visibility, VisibilityKind::Public);
        assert_eq!(struct_node.fields[1].visibility, VisibilityKind::Public);
    }

    // Check unit struct
    let unit_struct = code_graph
        .defined_types
        .iter()
        .find(|def| match def {
            TypeDefNode::Struct(s) => s.name == "UnitStruct",
            _ => false,
        })
        .expect("UnitStruct not found");

    if let TypeDefNode::Struct(struct_node) = unit_struct {
        assert_eq!(
            struct_node.fields.len(),
            0,
            "Expected 0 fields in UnitStruct"
        );
    }

    // =========== Enum Tests ===========
    let sample_enum = code_graph
        .defined_types
        .iter()
        .find(|def| match def {
            TypeDefNode::Enum(e) => e.name == "SampleEnum",
            _ => false,
        })
        .expect("SampleEnum not found");

    if let TypeDefNode::Enum(enum_node) = sample_enum {
        assert_eq!(enum_node.name, "SampleEnum");
        assert_eq!(enum_node.visibility, VisibilityKind::Public);

        // Check variants
        assert_eq!(
            enum_node.variants.len(),
            2,
            "Expected 2 variants in SampleEnum"
        );

        // First variant should be unit-like
        assert_eq!(enum_node.variants[0].name, "Variant1");
        assert_eq!(enum_node.variants[0].fields.len(), 0);
        assert_eq!(enum_node.variants[0].discriminant, None);

        // Second variant should have a single unnamed field
        assert_eq!(enum_node.variants[1].name, "Variant2");
        assert_eq!(enum_node.variants[1].fields.len(), 1);
        assert_eq!(enum_node.variants[1].fields[0].name, None);

        // Check generics and attributes
        assert_eq!(enum_node.generic_params.len(), 1);
        assert!(enum_node
            .attributes
            .iter()
            .any(|attr| attr.name == "derive"));
    }

    // Check enum with discriminants
    let module_enum = code_graph
        .defined_types
        .iter()
        .find(|def| match def {
            TypeDefNode::Enum(e) => e.name == "ModuleEnum",
            _ => false,
        })
        .expect("ModuleEnum not found");

    if let TypeDefNode::Enum(enum_node) = module_enum {
        assert_eq!(enum_node.variants.len(), 2);
        // Check discriminants
        assert!(enum_node.variants[0].discriminant.is_some());
        assert_eq!(enum_node.variants[0].discriminant.as_ref().unwrap(), "1");
        assert!(enum_node.variants[1].discriminant.is_some());
        assert_eq!(enum_node.variants[1].discriminant.as_ref().unwrap(), "2");
    }

    // =========== Trait Tests ===========
    let sample_trait = &code_graph.traits[0];
    assert_eq!(sample_trait.name, "SampleTrait");
    assert_eq!(sample_trait.visibility, VisibilityKind::Public);
    assert_eq!(sample_trait.generic_params.len(), 1);
    assert_eq!(sample_trait.methods.len(), 1);
    assert_eq!(sample_trait.methods[0].name, "trait_method");
    assert!(sample_trait.docstring.is_some());

    let default_trait = code_graph
        .traits
        .iter()
        .find(|t| t.name == "DefaultTrait")
        .expect("DefaultTrait not found");
    assert_eq!(default_trait.methods.len(), 1);
    assert_eq!(default_trait.methods[0].name, "default_method");
    // TODO: uncomment after adding `body` field to parser.rs
    // assert!(
    //     default_trait.methods[0].body.is_some(),
    //     "Expected default method to have a body"
    // );

    // =========== Function Tests ===========
    let sample_function = code_graph
        .functions
        .iter()
        .find(|f| f.name == "sample_function")
        .expect("sample_function not found");

    assert_eq!(sample_function.visibility, VisibilityKind::Public);
    assert_eq!(sample_function.parameters.len(), 2);
    assert!(sample_function.generic_params.len() > 0);
    assert!(sample_function.docstring.is_some());

    // Check parameter types
    assert!(sample_function.parameters[0].type_id != sample_function.parameters[0].type_id);

    // Check return type
    assert!(sample_function.return_type.is_some());

    // =========== Module Tests ===========
    let private_module = code_graph
        .modules
        .iter()
        .find(|m| m.name == "private_module")
        .expect("private_module not found");

    assert!(matches!(private_module.visibility, VisibilityKind::Restricted(_)));

    let public_module = code_graph
        .modules
        .iter()
        .find(|m| m.name == "public_module")
        .expect("public_module not found");

    assert_eq!(public_module.visibility, VisibilityKind::Public);

    // Check module contents through relations
    let items_in_public_module = code_graph
        .relations
        .iter()
        .filter(|r| r.kind == RelationKind::Contains && r.source == public_module.id)
        .count();

    assert!(
        items_in_public_module >= 2,
        "Expected at least 2 items in public_module"
    );

    // =========== Impl Tests ===========
    // Find impl of SampleTrait for SampleStruct
    let sample_trait_impl = code_graph
        .impls
        .iter()
        .find(|imp| {
            imp.trait_type.is_some()
                && code_graph
                    .traits
                    .iter()
                    .any(|t| Some(t.id) == imp.trait_type && t.name == "SampleTrait")
        })
        .expect("Implementation of SampleTrait not found");

    assert_eq!(sample_trait_impl.methods.len(), 1);
    assert_eq!(sample_trait_impl.methods[0].name, "trait_method");
    assert_eq!(sample_trait_impl.generic_params.len(), 1);

    // Find direct impl for SampleStruct
    let direct_impl = code_graph
        .impls
        .iter()
        .find(|imp| imp.trait_type.is_none() && imp.methods.iter().any(|m| m.name == "new"))
        .expect("Direct implementation for SampleStruct not found");

    assert_eq!(direct_impl.methods.len(), 2);
    assert!(direct_impl.methods.iter().any(|m| m.name == "use_field"));
}
