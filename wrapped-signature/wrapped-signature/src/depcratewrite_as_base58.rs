// Generated macro for write_as_base58 (function)
macro_rules! Depcratewrite_as_base58 {
() => {
// Module: crate
// Provides: {"write_as_base58"}
// Dependencies: {}
fn write_as_base58 (f : & mut fmt :: Formatter , s : & Signature) -> fmt :: Result { let mut out = [0u8 ; MAX_BASE58_SIGNATURE_LEN] ; let len = five8 :: encode_64 (& s . 0 , & mut out) as usize ; let as_str = unsafe { from_utf8_unchecked (& out [.. len]) } ; f . write_str (as_str) }
};
}
