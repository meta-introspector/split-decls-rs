macro_rules! deps {
    () => {
        DeValue!();
    };
}

macro_rules! TableEnumDeserializer {
    () => {
        deps!();
        # [doc = " Deserializes table values into enum variants."] pub (crate) struct TableEnumDeserializer < 'i > { value : DeValue < 'i > , span : core :: ops :: Range < usize > , }
    };
}

TableEnumDeserializer!()