// Generated macro for option (module)
macro_rules! Depcrate_serde_timestampoption {
() => {
// Module: crate::serde::timestamp
// Provides: {"option"}
// Dependencies: {}
# [doc = " Treat an `Option<OffsetDateTime>` as a [Unix timestamp] for the purposes of"] # [doc = " serde."] # [doc = ""] # [doc = " Use this module in combination with serde's [`#[with]`][with] attribute."] # [doc = ""] # [doc = " When deserializing, the offset is assumed to be UTC."] # [doc = ""] # [doc = " [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time"] # [doc = " [with]: https://serde.rs/field-attrs.html#with"] pub mod option { use super :: * ; # [doc = " Serialize an `Option<OffsetDateTime>` as its Unix timestamp"] # [inline] pub fn serialize < S : Serializer > (option : & Option < OffsetDateTime > , serializer : S ,) -> Result < S :: Ok , S :: Error > { option . map (OffsetDateTime :: unix_timestamp) . serialize (serializer) } # [doc = " Deserialize an `Option<OffsetDateTime>` from its Unix timestamp"] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D ,) -> Result < Option < OffsetDateTime > , D :: Error > { Option :: deserialize (deserializer) ? . map (OffsetDateTime :: from_unix_timestamp) . transpose () . map_err (| err | de :: Error :: invalid_value (de :: Unexpected :: Signed (err . value) , & err)) } }
};
}
