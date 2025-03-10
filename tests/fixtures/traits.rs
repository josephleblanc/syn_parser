// Sample with trait definitions and implementations

// Trait definition
pub trait SampleTrait {
    fn sample_method(&self) -> String;
}

// Trait with default implementation
pub trait DefaultTrait {
    fn default_method(&self) -> String {
        "Default implementation".to_string()
    }
}

// Trait implementation
impl SampleTrait for i32 {
    fn sample_method(&self) -> String {
        format!("Integer: {}", self)
    }
}

// Trait implementation with generics
impl<T> SampleTrait for Vec<T> {
    fn sample_method(&self) -> String {
        format!("Vector of size: {}", self.len())
    }
}

// Trait definition with docstring
/// This is a documented trait
pub trait DocumentedTrait {
    fn sample_method(&self) -> String;
}
