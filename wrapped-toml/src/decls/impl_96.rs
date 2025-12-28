macro_rules! deps {
    () => {
        Value!();
        MapEnumDeserializer!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl MapEnumDeserializer { pub (crate) fn new (value : Value) -> Self { Self { value } } }
    };
}

impl_96!()