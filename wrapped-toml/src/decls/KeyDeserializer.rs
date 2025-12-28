macro_rules! deps {
    () => {
        DeString!();
    };
}

macro_rules! KeyDeserializer {
    () => {
        deps!();
        pub (crate) struct KeyDeserializer < 'i > { span : Option < core :: ops :: Range < usize > > , key : DeString < 'i > , }
    };
}

KeyDeserializer!();