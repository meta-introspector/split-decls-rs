macro_rules! deps {
    () => {
        ValueSerializeVariant!();
        ValueSerializeMap!();
        SerializeMap!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl ValueSerializeVariant < ValueSerializeMap > { pub (crate) fn struct_ (variant : & 'static str , len : usize) -> Self { Self { variant , inner : ValueSerializeMap { ser : crate :: table :: SerializeMap :: with_capacity (len) , } , } } }
    };
}

impl_113!()