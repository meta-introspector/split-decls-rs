macro_rules! deps {
    () => {
        FieldSet!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl Eq for FieldSet { }
    };
}

impl_178!();