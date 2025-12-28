macro_rules! deps {
    () => {
        Item!();
        ValueDeserializer!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl ValueDeserializer { pub (crate) fn new (input : crate :: Item) -> Self { Self { input , validate_struct_keys : false , } } pub (crate) fn with_struct_key_validation (mut self) -> Self { self . validate_struct_keys = true ; self } }
    };
}

impl_317!()