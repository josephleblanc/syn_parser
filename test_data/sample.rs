fn sample_function() {
    println!("Hello, world!");
}

struct SampleStruct {
    field: i32,
}

enum SampleEnum {
    Variant1,
    Variant2(i32),
}

trait SampleTrait {
    fn sample_method(&self);
}

impl SampleTrait for SampleStruct {
    fn sample_method(&self) {
        println!("SampleStruct implementation of sample_method");
    }
}
