use std::path::PathBuf;
use syn_parser::parser::{analyze_code, save_graph, RelationKind, TypeDefNode, VisibilityKind};

#[test]
fn test_analyzer() {
    let input_path = PathBuf::from("test_data/sample.rs");
    let output_path = PathBuf::from("code_graph.ron");

    let code_graph_result = analyze_code(&input_path);
    assert!(code_graph_result.is_ok());

    let code_graph = code_graph_result.unwrap();
    save_graph(&code_graph, &output_path).expect("Failed to save graph");

    // Check the number of functions
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
    assert!(
        !code_graph.traits.is_empty(),
        "No traits found in the code graph"
    );
    assert_eq!(
        code_graph.traits.len(),
        1,
        "Expected 1 trait in the code graph"
    );

    // Check the number of impls
    assert!(
        !code_graph.impls.is_empty(),
        "No impls found in the code graph"
    );
    assert_eq!(
        code_graph.impls.len(),
        1,
        "Expected 1 impl in the code graph"
    );

    // Check the number of modules
    assert!(
        !code_graph.modules.is_empty(),
        "No modules found in the code graph"
    );
    assert_eq!(
        code_graph.modules.len(),
        1,
        "Expected 1 module in the code graph"
    );

    // Check the number of relations
    assert!(
        !code_graph.relations.is_empty(),
        "No relations found in the code graph"
    );
    assert_eq!(
        code_graph.relations.len(),
        3,
        "Expected 3 relations in the code graph"
    );

    // Check specific function details
    let function = &code_graph.functions[0];
    assert_eq!(function.name, "util_method", "Function name mismatch");
    assert_eq!(
        function.parameters.len(),
        0,
        "Expected 0 parameters for function util_method"
    );
    assert_eq!(
        function.return_type, None,
        "Expected no return type for function util_method"
    );
    assert_eq!(
        function.generic_params.len(),
        0,
        "Expected 0 generic parameters for function util_method"
    );
    assert_eq!(
        function.attributes.len(),
        0,
        "Expected 0 attributes for function util_method"
    );
    assert_eq!(
        function.docstring, None,
        "Expected no docstring for function util_method"
    );

    // Check specific defined type details
    let defined_type = &code_graph.defined_types[0];
    match defined_type {
        TypeDefNode::Struct(struct_node) => {
            assert_eq!(struct_node.name, "SampleStruct", "Struct name mismatch");
            assert_eq!(
                struct_node.fields.len(),
                0,
                "Expected 0 fields for struct SampleStruct"
            );
            assert_eq!(
                struct_node.generic_params.len(),
                1,
                "Expected 1 generic parameter for struct SampleStruct"
            );
            assert_eq!(
                struct_node.attributes.len(),
                0,
                "Expected 0 attributes for struct SampleStruct"
            );
            assert_eq!(
                struct_node.docstring, None,
                "Expected no docstring for struct SampleStruct"
            );
        }
        _ => panic!("Expected a struct, found a different type"),
    }

    // Check specific trait details
    let trait_node = &code_graph.traits[0];
    assert_eq!(trait_node.name, "UtilsTrait", "Trait name mismatch");
    assert_eq!(
        trait_node.methods.len(),
        1,
        "Expected 1 method for trait UtilsTrait"
    );
    assert_eq!(
        trait_node.generic_params.len(),
        0,
        "Expected 0 generic parameters for trait UtilsTrait"
    );
    assert_eq!(
        trait_node.super_traits.len(),
        0,
        "Expected 0 super traits for trait UtilsTrait"
    );
    assert_eq!(
        trait_node.attributes.len(),
        0,
        "Expected 0 attributes for trait UtilsTrait"
    );
    assert_eq!(
        trait_node.docstring, None,
        "Expected no docstring for trait UtilsTrait"
    );

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
    let module_node = &code_graph.modules[0];
    assert_eq!(module_node.name, "utils", "Module name mismatch");
    assert_eq!(
        module_node.visibility,
        VisibilityKind::Public,
        "Expected public visibility for module utils"
    );
    assert_eq!(
        module_node.attributes.len(),
        0,
        "Expected 0 attributes for module utils"
    );
    assert_eq!(
        module_node.docstring, None,
        "Expected no docstring for module utils"
    );
    assert_eq!(
        module_node.submodules.len(),
        0,
        "Expected 0 submodules for module utils"
    );
    assert_eq!(
        module_node.items.len(),
        2,
        "Expected 2 items for module utils"
    );
    assert_eq!(
        module_node.imports.len(),
        0,
        "Expected 0 imports for module utils"
    );
    assert_eq!(
        module_node.exports.len(),
        0,
        "Expected 0 exports for module utils"
    );

    // Check specific relation details
    let relation = &code_graph.relations[0];
    assert_eq!(
        relation.source, impl_node.id,
        "Expected relation source to match impl id"
    );
    assert_eq!(
        relation.target, function.id,
        "Expected relation target to match function id"
    );
    assert_eq!(
        relation.kind,
        RelationKind::ImplementsTrait,
        "Expected relation kind to be ImplementsTrait"
    );

    let relation = &code_graph.relations[1];
    assert_eq!(
        relation.source, impl_node.id,
        "Expected relation source to match impl id"
    );
    assert_eq!(
        relation.target, code_graph.traits[0].id,
        "Expected relation target to match trait id"
    );
    assert_eq!(
        relation.kind,
        RelationKind::ImplementsTrait,
        "Expected relation kind to be ImplementsTrait"
    );

    let relation = &code_graph.relations[2];
    assert_eq!(
        relation.source, impl_node.id,
        "Expected relation source to match impl id"
    );
    assert_eq!(
        relation.target,
        code_graph.defined_types[0].id(),
        "Expected relation target to match defined type id"
    );
    assert_eq!(
        relation.kind,
        RelationKind::ImplementsFor,
        "Expected relation kind to be ImplementsFor"
    );

    println!("Code graph saved to {:?}", output_path);
}
