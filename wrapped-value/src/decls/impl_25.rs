macro_rules! deps {
    () => {
        NameDeserializer!();
        Name!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl NameDeserializer { # [inline] fn new (value : Name) -> Self { NameDeserializer { value } } }
    };
}

impl_25!();