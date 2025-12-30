// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_timestamp_nanosecondsdeserialize {
() => {
// Module: crate::serde::timestamp::nanoseconds
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an `OffsetDateTime` from its Unix timestamp with nanoseconds"] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { OffsetDateTime :: from_unix_timestamp_nanos (< _ > :: deserialize (deserializer) ?) . map_err (| err | de :: Error :: invalid_value (de :: Unexpected :: Signed (err . value) , & err)) }
};
}
