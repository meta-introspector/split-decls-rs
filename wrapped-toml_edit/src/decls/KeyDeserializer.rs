macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! KeyDeserializer {
    () => {
        deps!();
        pub (crate) struct KeyDeserializer { span : Option < std :: ops :: Range < usize > > , key : crate :: Key , }
    };
}

KeyDeserializer!();