macro_rules! deps {
    () => {
        KeyDeserializer!();
        Key!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl KeyDeserializer { pub (crate) fn new (key : crate :: Key , span : Option < std :: ops :: Range < usize > >) -> Self { Self { span , key } } }
    };
}

impl_295!();