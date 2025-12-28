macro_rules! ValueSerializeVariant {
    () => {
        pub (crate) struct ValueSerializeVariant < T > { variant : & 'static str , inner : T , }
    };
}

ValueSerializeVariant!();