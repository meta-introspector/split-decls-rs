// Generated macro for serialize (function)
macro_rules! Depcrate_serde_timestamp_milliseconds_i64serialize {
() => {
// Module: crate::serde::timestamp::milliseconds_i64
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an `OffsetDateTime` as its Unix timestamp with milliseconds"] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { let timestamp = (datetime . unix_timestamp_nanos () / 1_000_000) . truncate :: < i64 > () ; timestamp . serialize (serializer) }
};
}
