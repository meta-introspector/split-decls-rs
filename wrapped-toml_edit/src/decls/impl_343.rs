macro_rules! deps {
    () => {
        SerializeValueArray!();
        SerializeTupleVariant!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl SerializeTupleVariant { pub (crate) fn tuple (variant : & 'static str , len : usize) -> Self { Self { variant , inner : SerializeValueArray :: seq (Some (len)) , } } }
    };
}

impl_343!()