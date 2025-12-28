macro_rules! deps {
    () => {
        Table!();
        Value!();
        IntoIter!();
    };
}

macro_rules! MapDeserializer {
    () => {
        deps!();
        pub (crate) struct MapDeserializer { iter : < Table as IntoIterator > :: IntoIter , value : Option < (String , Value) > , }
    };
}

MapDeserializer!()