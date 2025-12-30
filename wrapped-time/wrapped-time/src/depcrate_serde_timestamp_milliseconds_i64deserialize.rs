// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_timestamp_milliseconds_i64deserialize {
() => {
// Module: crate::serde::timestamp::milliseconds_i64
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an `OffsetDateTime` from its Unix timestamp with milliseconds"] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { let value : i64 = < _ > :: deserialize (deserializer) ? ; OffsetDateTime :: from_unix_timestamp_nanos (value . extend :: < i128 > () * 1_000_000) . map_err (| err | de :: Error :: invalid_value (de :: Unexpected :: Signed (err . value) , & err)) }
};
}
