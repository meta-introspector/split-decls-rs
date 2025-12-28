macro_rules! deps {
    () => {
        Field!();
        FieldSet!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl Clone for Field { fn clone (& self) -> Self { Field { i : self . i , fields : FieldSet { names : self . fields . names , callsite : self . fields . callsite () , } , } } }
    };
}

impl_173!()