// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_timestampdeserialize {
() => {
// Module: crate::serde::timestamp
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an `OffsetDateTime` from its Unix timestamp"] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { OffsetDateTime :: from_unix_timestamp (< _ > :: deserialize (deserializer) ?) . map_err (| err | de :: Error :: invalid_value (de :: Unexpected :: Signed (err . value) , & err)) }
};
}
