macro_rules! deps {
    () => {
        Item!();
        ArrayDeserializer!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl ArrayDeserializer { pub (crate) fn new (input : Vec < crate :: Item > , span : Option < std :: ops :: Range < usize > >) -> Self { Self { input , span } } }
    };
}

impl_278!()