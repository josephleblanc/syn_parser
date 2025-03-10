// Sample with various function types

// Regular function
pub fn regular_function() {
    println!("Regular function");
}

// Function with parameters
pub fn function_with_params(x: i32, y: i32) -> i32 {
    x + y
}

// Generic function
pub fn generic_function<T>(arg: T) -> T {
    arg
}

// Function with attributes
#[cfg(test)]
pub fn attributed_function() {
    println!("Attributed function");
}

// Function with docstring
/// This is a documented function
pub fn documented_function() {
    println!("Documented function");
}

// Unsafe function
pub unsafe fn unsafe_function() {
    println!("Unsafe function");
}

// Function with lifetime annotations
pub fn lifetime_function<'a>(arg: &'a str) -> &'a str {
    arg
}
