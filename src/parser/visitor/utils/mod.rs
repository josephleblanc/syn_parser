//! Shared utilities for visitor pattern implementation
//!
//! Contains helper functions and common utilities used across
//! the code graph parser visitor implementation.

pub mod attributes;
pub mod docs;
pub mod generics;

// Re-export main utility functions at module level
pub use self::attributes::extract_attributes;
pub use self::docs::extract_docstring;
pub use self::generics::process_generics;
pub use self::attributes::ParsedAttribute;
