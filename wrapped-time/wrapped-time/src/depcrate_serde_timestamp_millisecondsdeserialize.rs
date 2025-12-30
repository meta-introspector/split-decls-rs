// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_timestamp_millisecondsdeserialize {
() => {
// Module: crate::serde::timestamp::milliseconds
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an `OffsetDateTime` from its Unix timestamp with milliseconds"] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { let value : i128 = < _ > :: deserialize (deserializer) ? ; OffsetDateTime :: from_unix_timestamp_nanos (value * 1_000_000) . map_err (| err | de :: Error :: invalid_value (de :: Unexpected :: Signed (err . value) , & err)) }
};
}
