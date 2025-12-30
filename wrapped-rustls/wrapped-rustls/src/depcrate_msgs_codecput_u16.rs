// Generated macro for put_u16 (function)
macro_rules! Depcrate_msgs_codecput_u16 {
() => {
// Module: crate::msgs::codec
// Provides: {"put_u16"}
// Dependencies: {}
pub (crate) fn put_u16 (v : u16 , out : & mut [u8]) { let out : & mut [u8 ; 2] = (& mut out [.. 2]) . try_into () . unwrap () ; * out = u16 :: to_be_bytes (v) ; }
};
}
