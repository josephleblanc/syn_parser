// Use the lib.rs exports instead of direct imports from parser
use crate::parser::analyze_code;
use crate::serialization::ron::save_to_ron;
use std::path::Path;

const TARGET_FILE: &str = "src/main.rs";
const RON_SAVE_FILE: &str = "data/graph.ron";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new(TARGET_FILE);
    let save_path = Path::new(RON_SAVE_FILE);

    let code_graph = analyze_code(target_path)?;
    Ok(save_to_ron(&code_graph, save_path)?)
}
