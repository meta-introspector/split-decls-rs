macro_rules! deps {
    () => {
        DeArray!();
    };
}

macro_rules! ArrayDeserializer {
    () => {
        deps!();
        pub (crate) struct ArrayDeserializer < 'i > { input : DeArray < 'i > , span : core :: ops :: Range < usize > , }
    };
}

ArrayDeserializer!();