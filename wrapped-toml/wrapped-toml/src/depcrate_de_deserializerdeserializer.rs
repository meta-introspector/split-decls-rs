// Generated macro for Deserializer (struct)
macro_rules! Depcrate_de_deserializerDeserializer {
() => {
// Module: crate::de::deserializer
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " Deserialization for TOML [documents][crate::Table]."] # [doc = ""] # [doc = " To deserializes TOML values, instead of documents, see [`ValueDeserializer`]."] pub struct Deserializer < 'i > { span : core :: ops :: Range < usize > , root : DeTable < 'i > , raw : Option < & 'i str > , }
};
}
