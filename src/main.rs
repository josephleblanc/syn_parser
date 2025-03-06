mod parser;
use crate::parser::analyze_code;
use crate::parser::save_graph;
use std::path::Path;

const TARGET_FILE: &str = "/home/brasides/code/rag_workspace/example_traverse_target/src/main.rs";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new(TARGET_FILE);
    let save_path = Path::new("./data/graph.ron");

    let code_graph = analyze_code(target_path)?;
    Ok(save_graph(&code_graph, save_path)?)
}
