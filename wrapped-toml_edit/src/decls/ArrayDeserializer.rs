macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! ArrayDeserializer {
    () => {
        deps!();
        pub (crate) struct ArrayDeserializer { input : Vec < crate :: Item > , span : Option < std :: ops :: Range < usize > > , }
    };
}

ArrayDeserializer!()