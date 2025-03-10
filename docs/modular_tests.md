
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                      Making Tests More Modular for the Code Graph Project                       ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

Based on the code you've shared, I can see you have a comprehensive parser that
builds a code graph from Rust source files. Your current testing approach in
tests/parser_tests.rs is a single large test function that checks many aspects
of the parser. Let's make this more modular and maintainable.

                                     Current Testing Structure                                     

Your current test:

 • Uses a single sample file (tests/data/sample.rs)
 • Has one large test function that checks many different aspects
 • Mixes different concerns (parsing, serialization, entity counts, specific entity details)
 • Makes assertions about many different types of nodes in the same test

                                Proposed Modular Testing Structure                                 

Here's how I recommend organizing your tests:

                                     1. Split Tests by Concern                                     

Create separate test modules for different aspects of your code:

 tests/
 ├── parser/
 │   ├── mod.rs
 │   ├── basic_parsing.rs      # Basic parsing functionality
 │   ├── functions_tests.rs    # Function parsing tests
 │   ├── structs_tests.rs      # Struct parsing tests
 │   ├── enums_tests.rs        # Enum parsing tests
 │   ├── traits_tests.rs       # Trait parsing tests
 │   ├── impls_tests.rs        # Implementation tests
 │   ├── modules_tests.rs      # Module parsing tests
 │   ├── macros_tests.rs       # Macro parsing tests
 │   └── visibility_tests.rs   # Visibility handling tests
 ├── serialization/
 │   ├── mod.rs
 │   ├── ron_tests.rs          # RON serialization tests
 │   └── json_tests.rs         # JSON serialization tests (if implemented)
 └── integration/
     ├── mod.rs
     └── full_graph_tests.rs   # End-to-end tests

                                      2. Create Test Fixtures                                      

Instead of relying on a single large sample file, create smaller, focused test fixtures:

 tests/fixtures/
 ├── functions.rs        # Sample with various function types
 ├── structs.rs          # Sample with various struct types
 ├── enums.rs            # Sample with various enum types
 ├── traits.rs           # Sample with trait definitions and implementations
 ├── modules.rs          # Sample with module structure
 ├── visibility.rs       # Sample with different visibility modifiers
 └── macros.rs           # Sample with macro definitions

                                    3. Create Helper Functions                                     

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

                                      4. Example Test Module                                       

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

                                       5. Integration Tests                                        

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

This modular approach will make your tests more maintainable as your project
grows and will help ensure that new features don't break existing
functionality.
