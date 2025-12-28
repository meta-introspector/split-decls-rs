macro_rules! deps {
    () => {
        DeArray!();
        ArrayDeserializer!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'i > ArrayDeserializer < 'i > { pub (crate) fn new (input : DeArray < 'i > , span : core :: ops :: Range < usize >) -> Self { Self { input , span } } }
    };
}

impl_118!();