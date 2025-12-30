// Generated macro for encode_len (function)
macro_rules! Depcrate_k12encode_len {
() => {
// Module: crate::k12
// Provides: {"encode_len"}
// Dependencies: {}
fn encode_len (len : usize) -> EncodedLen { let len_view = (len as u64) . to_be_bytes () ; let offset = len_view . iter () . position (| i | * i != 0) . unwrap_or (8) ; let mut buffer = [0u8 ; 9] ; buffer [.. 8] . copy_from_slice (& len_view) ; buffer [8] = 8 - offset as u8 ; EncodedLen { offset , buffer } }
};
}
