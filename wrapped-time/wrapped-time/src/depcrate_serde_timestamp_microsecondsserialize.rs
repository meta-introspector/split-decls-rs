// Generated macro for serialize (function)
macro_rules! Depcrate_serde_timestamp_microsecondsserialize {
() => {
// Module: crate::serde::timestamp::microseconds
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an `OffsetDateTime` as its Unix timestamp with microseconds"] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { let timestamp = datetime . unix_timestamp_nanos () / 1_000 ; timestamp . serialize (serializer) }
};
}
