macro_rules! deps {
    () => {
        Table!();
        SerializeTable!();
        SerializeDatetime!();
        Style!();
        SerializeMap!();
        Error!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < 'd > SerializeMap < 'd > { pub (crate) fn map (dst : & 'd mut String , style : Style) -> Result < Self , Error > { Ok (Self :: Table (SerializeTable :: map (dst , style) ?)) } pub (crate) fn struct_ (name : & 'static str , dst : & 'd mut String , style : Style ,) -> Result < Self , Error > { if toml_datetime :: ser :: is_datetime (name) { Ok (Self :: Datetime (SerializeDatetime :: new (dst))) } else { Ok (Self :: map (dst , style) ?) } } }
    };
}

impl_339!()