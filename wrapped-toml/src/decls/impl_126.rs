macro_rules! deps {
    () => {
        DeString!();
        KeyDeserializer!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'i > KeyDeserializer < 'i > { pub (crate) fn new (key : DeString < 'i > , span : Option < core :: ops :: Range < usize > >) -> Self { KeyDeserializer { span , key } } }
    };
}

impl_126!();