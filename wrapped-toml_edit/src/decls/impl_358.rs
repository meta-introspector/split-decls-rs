macro_rules! deps {
    () => {
        SerializeInlineTable!();
        SerializeDatetime!();
        SerializeMap!();
        Table!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl SerializeMap { pub (crate) fn map (len : Option < usize >) -> Self { Self :: Table (SerializeInlineTable :: map (len)) } pub (crate) fn struct_ (name : & 'static str , len : Option < usize >) -> Self { if toml_datetime :: ser :: is_datetime (name) { Self :: Datetime (SerializeDatetime :: new ()) } else { Self :: map (len) } } }
    };
}

impl_358!()