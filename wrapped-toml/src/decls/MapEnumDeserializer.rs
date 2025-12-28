macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! MapEnumDeserializer {
    () => {
        deps!();
        # [doc = " Deserializes table values into enum variants."] pub (crate) struct MapEnumDeserializer { value : Value , }
    };
}

MapEnumDeserializer!()