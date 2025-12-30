// Generated macro for option (module)
macro_rules! Depcrate_serde_timestamp_milliseconds_i64option {
() => {
// Module: crate::serde::timestamp::milliseconds_i64
// Provides: {"option"}
// Dependencies: {}
# [doc = " Treat an `Option<OffsetDateTime>` as a [Unix timestamp] with milliseconds"] # [doc = " for the purposes of serde."] # [doc = ""] # [doc = " Use this module in combination with serde's [`#[with]`][with] attribute."] # [doc = ""] # [doc = " When deserializing, the offset is assumed to be UTC."] # [doc = ""] # [doc = " [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time"] # [doc = " [with]: https://serde.rs/field-attrs.html#with"] pub mod option { use super :: * ; # [doc = " Serialize an `Option<OffsetDateTime>` as its Unix timestamp with milliseconds"] # [inline] pub fn serialize < S : Serializer > (option : & Option < OffsetDateTime > , serializer : S ,) -> Result < S :: Ok , S :: Error > { option . map (| timestamp | (timestamp . unix_timestamp_nanos () / 1_000_000) . truncate :: < i64 > ()) . serialize (serializer) } # [doc = " Deserialize an `Option<OffsetDateTime>` from its Unix timestamp with milliseconds"] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D ,) -> Result < Option < OffsetDateTime > , D :: Error > { Option :: deserialize (deserializer) ? . map (| value : i64 | { OffsetDateTime :: from_unix_timestamp_nanos (value . extend :: < i128 > () * 1_000_000) }) . transpose () . map_err (| err | de :: Error :: invalid_value (de :: Unexpected :: Signed (err . value) , & err)) } }
};
}
