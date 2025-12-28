macro_rules! deps {
    () => {
        ValueDeserializer!();
        DeTable!();
    };
}

macro_rules! Deserializer {
    () => {
        deps!();
        # [doc = " Deserialization for TOML [documents][crate::Table]."] # [doc = ""] # [doc = " To deserializes TOML values, instead of documents, see [`ValueDeserializer`]."] pub struct Deserializer < 'i > { span : core :: ops :: Range < usize > , root : DeTable < 'i > , raw : Option < & 'i str > , }
    };
}

Deserializer!()