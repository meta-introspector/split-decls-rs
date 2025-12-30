// Generated macro for serialize (function)
macro_rules! Depcrate_serde_timestamp_nanosecondsserialize {
() => {
// Module: crate::serde::timestamp::nanoseconds
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an `OffsetDateTime` as its Unix timestamp with nanoseconds"] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { datetime . unix_timestamp_nanos () . serialize (serializer) }
};
}
