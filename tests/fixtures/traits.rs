pub trait SampleTrait {
    fn sample_method() {}
}
// Trait with associated types and where clauses
pub trait ComplexTrait<T: Clone>
where
    T: Send,
{
    type AssocType;
    fn method(&self) -> Result<T, Self::AssocType>;

    #[allow(unused)]
    fn default_impl(&self) -> String {
        "default".into()
    }
}

pub trait DefaultTrait {
    fn default_method(&self) -> String {
        "Default implementation".to_string()
    }
}

pub trait GenericTrait<T> {
    fn generic_method(&self, param: T) -> T;
}

pub trait assoc_type_trait {
    type AssocType;
    fn method_with_assoc(&self) -> Self::AssocType;
}

trait PrivateTrait {
    fn private_method(&self) -> i32;
}
