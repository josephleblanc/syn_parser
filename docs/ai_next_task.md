
# AI Next Task

Remember to complete these tasks in small steps whenever possible!
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                      Making Tests More Modular for the Code Graph Project                       ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

                                    3. Create Helper Functions ✅                                 

Create test helpers to reduce duplication:

 // tests/common/mod.rs
 use std::path::Path;
 use syn_parser::parser::*;

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

 // Similar helpers for other node types

                                      4. Example Test Module  ✅                                    

Here's how a test module for structs might look:

 // tests/parser/structs_tests.rs
 use crate::common::*;
 use syn_parser::parser::nodes::*;
 use syn_parser::parser::types::*;

 #[test]
 fn test_regular_struct_parsing() {
     let graph = parse_fixture("structs.rs");

     let sample_struct = find_struct_by_name(&graph, "SampleStruct")                               
         .expect("SampleStruct not found");                                                        
                                                                                                   
     assert_eq!(sample_struct.name, "SampleStruct");                                               
     assert_eq!(sample_struct.visibility, VisibilityKind::Public);                                 
     assert_eq!(sample_struct.fields.len(), 1);                                                    
     assert_eq!(sample_struct.fields[0].name, Some("field".to_string()));                          
 }

 #[test]
 fn test_tuple_struct_parsing() {
     let graph = parse_fixture("structs.rs");

     let tuple_struct = find_struct_by_name(&graph, "TupleStruct")                                 
         .expect("TupleStruct not found");                                                         
                                                                                                   
     assert_eq!(tuple_struct.fields.len(), 2);                                                     
     assert_eq!(tuple_struct.fields[0].visibility, VisibilityKind::Public);                        
 }

 #[test]
 fn test_unit_struct_parsing() {
     let graph = parse_fixture("structs.rs");

     let unit_struct = find_struct_by_name(&graph, "UnitStruct")                                   
         .expect("UnitStruct not found");                                                          
                                                                                                   
     assert_eq!(unit_struct.fields.len(), 0);                                                      
 }

 #[test]
 fn test_struct_with_generics() {
     let graph = parse_fixture("structs.rs");

     let generic_struct = find_struct_by_name(&graph, "GenericStruct")                             
         .expect("GenericStruct not found");                                                       
                                                                                                   
     assert_eq!(generic_struct.generic_params.len(), 1);                                           
     if let GenericParamKind::Type { name, .. } = &generic_struct.generic_params[0].kind {         
         assert_eq!(name, "T");                                                                    
     } else {                                                                                      
         panic!("Expected Type generic parameter");                                                
     }                                                                                             
 }

 #[test]
 fn test_struct_with_attributes() {
     let graph = parse_fixture("structs.rs");

     let attributed_struct = find_struct_by_name(&graph, "AttributedStruct")                       
         .expect("AttributedStruct not found");                                                    
                                                                                                   
     assert!(attributed_struct.attributes.iter().any(|attr| attr.name == "derive"));               
 }

 #[test]
 fn test_struct_with_docstring() {
     let graph = parse_fixture("structs.rs");

     let documented_struct = find_struct_by_name(&graph, "DocumentedStruct")                       
         .expect("DocumentedStruct not found");                                                    
                                                                                                   
     assert!(documented_struct.docstring.is_some());                                               
     assert!(documented_struct.docstring.as_ref().unwrap().contains("documented struct"));         
 }

                                       5. Integration Tests  ✅                                    

Keep some integration tests that verify the entire system works together:

 // tests/integration/full_graph_tests.rs
 use std::path::Path;
 use syn_parser::parser::*;
 use syn_parser::serialization::ron::save_to_ron;

 #[test]
 fn test_full_graph_generation_and_serialization() {
     let input_path = Path::new("tests/data/sample.rs");
     let output_path = Path::new("tests/data/code_graph.ron");

     let code_graph = analyze_code(&input_path).expect("Failed to analyze code");                  
     save_to_ron(&code_graph, &output_path).expect("Failed to save graph");                        
                                                                                                   
     // Basic sanity checks                                                                        
     assert!(!code_graph.functions.is_empty());                                                    
     assert!(!code_graph.defined_types.is_empty());                                                
     assert!(!code_graph.traits.is_empty());                                                       
     assert!(!code_graph.impls.is_empty());                                                        
     assert!(!code_graph.modules.is_empty());                                                      
 }

                                       Implementation Steps                                        

 1. Create the directory structure for the new test organization

 2. Create test fixtures - smaller, focused Rust files for testing specific
    features

 3. Create helper functions in a common module

 4. Split the existing test into smaller, focused tests

 5. Add new tests for edge cases and features not currently covered

                                     Benefits of This Approach                                     

 1. Better organization: Tests are grouped by the feature they're testing
 2. Easier maintenance: When you modify a specific part of the parser, you
     know which tests to check
 3. Better test isolation: Each test focuses on one aspect, making failures
     more specific
 4. Improved documentation: Tests serve as examples of how different Rust
     constructs are parsed
 5. Easier to extend: Adding tests for new features is simpler with the
     modular structure

                                     Implementation Progress                                       

So far, we've implemented:

 1. ✅ Directory structure for modular tests
 2. ✅ Common helper functions in tests/common/mod.rs
 3. ✅ Function parsing tests with fixtures

Next steps:

 4. Implement Struct parsing tests
 5. Implement Enum parsing tests
 1. Implement trait parsing tests
 2. Implement impl block parsing tests
 3. Implement module parsing tests
 4. Implement macro parsing tests
 5. Implement visibility tests
 6. Implement serialization tests
 7. Implement integration tests

Each test module follows a similar pattern:

 1. Create a fixture file with various examples of the Rust construct
 2. Create test functions that verify each aspect of parsing
 3. Use helper functions to reduce code duplication
 4. Make assertions about the parsed code graph

This modular approach will make your tests more maintainable as your project
grows and will help ensure that new features don't break existing
functionality.

Remember to complete these tasks in small steps whenever possible!
Break each task into sub-tasks to achieve the goal of that task.
