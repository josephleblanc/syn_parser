use crate::common::*;
use syn_parser::parser::nodes::NodeId;
use syn_parser::parser::types::TypeId;

#[test]
fn test_id_type_safety() {
    // Verify explicit conversion works but types remain distinct
    let node_id = NodeId::from(1);
    let type_id = TypeId::from(1);
    
    // Same underlying value but different types
    assert_eq!(node_id.as_usize(), type_id.as_usize());
    assert_ne!(node_id, type_id.as_node_id().unwrap());
    
    // Verify type-safe comparisons
    assert_ne!(
        node_id.to_string(),
        type_id.to_string(),
        "String representations should differ"
    );
    
    // Verify hash differences
    use std::collections::HashSet;
    let mut ids = HashSet::new();
    ids.insert(node_id);
    assert!(!ids.contains(&type_id.as_node_id().unwrap()));
}
