macro_rules! deps {
    () => {
        SerializeValueArray!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl SerializeValueArray { pub (crate) fn seq (len : Option < usize >) -> Self { let mut values = Vec :: new () ; if let Some (len) = len { values . reserve (len) ; } Self { values } } }
    };
}

impl_338!()