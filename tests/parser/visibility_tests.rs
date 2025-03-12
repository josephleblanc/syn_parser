use crate::common::*;
use syn_parser::parser::visitor::state::VisitorState;
use syn_parser::parser::visitor::CodeVisitor;
use syn_parser::parser::visitor::ModuleVisitor;

// TODO: Fix these up later
// #[test]
// fn test_module_visibility() {
//     let mut state = VisitorState::new();
//     let mut visitor = CodeVisitor::new(&mut state);
//
//     let mod_code = syn::parse_str(
//         r#"
//         pub mod public_module {
//             mod private_module {
//                 pub struct PublicStruct;
//             }
//         }
//     "#,
//     )
//     .unwrap();
//
//     visitor.process_module(&mod_code);
//
//     let public_module = state
//         .code_graph
//         .modules
//         .iter()
//         .find(|m| m.name == "public_module")
//         .unwrap();
//     assert_eq!(public_module.visibility, VisibilityKind::Public);
//
//     let private_module = state
//         .code_graph
//         .modules
//         .iter()
//         .find(|m| m.name == "private_module")
//         .unwrap();
//     assert_eq!(
//         private_module.visibility,
//         VisibilityKind::Restricted(vec!["super".to_string()])
//     );
// }
