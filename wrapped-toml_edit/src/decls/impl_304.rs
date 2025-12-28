macro_rules! deps {
    () => {
        TableDeserializer!();
        KeyValuePairs!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl TableDeserializer { pub (crate) fn new (items : crate :: table :: KeyValuePairs , span : Option < std :: ops :: Range < usize > > ,) -> Self { Self { span , items } } }
    };
}

impl_304!();