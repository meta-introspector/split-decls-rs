macro_rules! deps {
    () => {
        Item!();
        TableEnumDeserializer!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl TableEnumDeserializer { pub (crate) fn new (value : crate :: Item) -> Self { Self { value } } }
    };
}

impl_313!()