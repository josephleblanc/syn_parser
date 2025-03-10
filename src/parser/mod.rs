mod graph;
mod nodes;
mod relations;
mod types;
mod visitor;

pub use self::graph::CodeGraph;
pub use self::serialization::save_graph;
pub use self::types::TypeId;
pub use self::visitor::analyze_code;

// Internal module for serialization functions
mod serialization {
    use super::graph::CodeGraph;
    use crate::serialization::ron::save_to_ron;
    use std::path::Path;

    pub fn save_graph(code_graph: &CodeGraph, output_path: &Path) -> std::io::Result<()> {
        save_to_ron(code_graph, output_path)
    }
}
