pub struct SampleStruct {
    pub field: String,
}

pub trait SampleTrait {
    fn sample_method(&self) -> String;
}

impl SampleTrait for SampleStruct {
    fn sample_method(&self) -> String {
        "Sample implementation".to_string()
    }
}

impl SampleStruct {
    pub fn new(field: String) -> Self {
        SampleStruct { field }
    }

    pub fn get_field(&self) -> &str {
        &self.field
    }
}

pub struct GenericStruct<T> {
    pub field: T,
}

impl<T> GenericStruct<T> {
    pub fn new(field: T) -> Self {
        GenericStruct { field }
    }

    pub fn get_field(&self) -> &T {
        &self.field
    }
}

pub trait GenericTrait<T> {
    fn generic_method(&self, param: T) -> T;
}

impl<T> GenericTrait<T> for GenericStruct<T> {
    fn generic_method(&self, param: T) -> T {
        param
    }
}
