#![allow(dead_code)]
use super::test_dir;
use std::fmt::Debug;

trait PrintDebug: Debug {
    fn print_debug(&self) {
        println!("{:?}", self);
    }
}

impl<T: Debug> PrintDebug for T {}

#[derive(Debug)]
struct SampleStruct<T> {
    field: T,
}

impl<T: Debug> SampleStruct<T> {
    fn new(field: T) -> Self {
        SampleStruct { field }
    }
}

#[derive(Debug)]
enum SampleEnum<T: Debug> {
    Variant1,
    Variant2(T),
}

trait SampleTrait<T: Debug> {
    fn sample_method(&self, arg: T);
}

impl<T: Debug> SampleTrait<T> for SampleStruct<T> {
    fn sample_method(&self, arg: T) {
        println!(
            "SampleStruct implementation of sample_method with arg: {:?}",
            arg
        );
    }
}

struct NestedStruct {
    nested_field: i32,
}

impl std::fmt::Debug for NestedStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedStruct")
            .field("nested_field", &self.nested_field)
            .finish()
    }
}

fn sample_function<T: Debug>(arg: T) {
    println!("Hello, world with arg: {:?}", arg);
}

impl<T> test_dir::example_file::UtilsTrait for SampleStruct<T> {
    fn util_method(&self) {
        println!("Util method for SampleStruct");
    }
}
