// Generated macro for decode_unix_timestamp_millis (function)
macro_rules! Depcrate_timestampdecode_unix_timestamp_millis {
() => {
// Module: crate::timestamp
// Provides: {"decode_unix_timestamp_millis"}
// Dependencies: {}
pub (crate) const fn decode_unix_timestamp_millis (uuid : & Uuid) -> u64 { let bytes = uuid . as_bytes () ; let millis : u64 = (bytes [0] as u64) << 40 | (bytes [1] as u64) << 32 | (bytes [2] as u64) << 24 | (bytes [3] as u64) << 16 | (bytes [4] as u64) << 8 | (bytes [5] as u64) ; millis }
};
}
