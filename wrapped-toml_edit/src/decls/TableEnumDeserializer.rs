macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! TableEnumDeserializer {
    () => {
        deps!();
        # [doc = " Deserializes table values into enum variants."] pub (crate) struct TableEnumDeserializer { value : crate :: Item , }
    };
}

TableEnumDeserializer!();