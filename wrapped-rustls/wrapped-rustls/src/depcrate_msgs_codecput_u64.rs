// Generated macro for put_u64 (function)
macro_rules! Depcrate_msgs_codecput_u64 {
() => {
// Module: crate::msgs::codec
// Provides: {"put_u64"}
// Dependencies: {}
pub (crate) fn put_u64 (v : u64 , bytes : & mut [u8]) { let bytes : & mut [u8 ; 8] = (& mut bytes [.. 8]) . try_into () . unwrap () ; * bytes = u64 :: to_be_bytes (v) ; }
};
}
