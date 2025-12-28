macro_rules! deps {
    () => {
        Table!();
        SerializeInlineTable!();
        SerializeDatetime!();
    };
}

macro_rules! SerializeMap {
    () => {
        deps!();
        # [doc (hidden)] # [allow (clippy :: large_enum_variant)] pub enum SerializeMap { Datetime (SerializeDatetime) , Table (SerializeInlineTable) , }
    };
}

SerializeMap!();