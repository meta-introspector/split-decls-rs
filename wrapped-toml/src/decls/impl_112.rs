macro_rules! deps {
    () => {
        ValueSerializeVariant!();
        ValueSerializeVec!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl ValueSerializeVariant < ValueSerializeVec > { pub (crate) fn tuple (variant : & 'static str , len : usize) -> Self { Self { variant , inner : ValueSerializeVec { vec : Vec :: with_capacity (len) , } , } } }
    };
}

impl_112!();