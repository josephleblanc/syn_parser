pub mod parser;
pub mod serialization;
// pub mod analysis; // For future code analysis features

// Re-export key items for easier access
pub use parser::{CodeGraph, analyze_code};
pub use serialization::ron::save_to_ron;
