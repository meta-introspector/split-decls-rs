// Generated macro for serialize (function)
macro_rules! Depcrate_serde_timestampserialize {
() => {
// Module: crate::serde::timestamp
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an `OffsetDateTime` as its Unix timestamp"] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { datetime . unix_timestamp () . serialize (serializer) }
};
}
