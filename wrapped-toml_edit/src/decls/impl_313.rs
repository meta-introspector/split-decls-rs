macro_rules! deps {
    () => {
        TableEnumDeserializer!();
        Item!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl TableEnumDeserializer { pub (crate) fn new (value : crate :: Item) -> Self { Self { value } } }
    };
}

impl_313!();