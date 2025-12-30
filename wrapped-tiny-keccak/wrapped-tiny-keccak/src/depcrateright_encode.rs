// Generated macro for right_encode (function)
macro_rules! Depcrateright_encode {
() => {
// Module: crate
// Provides: {"right_encode"}
// Dependencies: {}
fn right_encode (len : usize) -> EncodedLen { let mut buffer = [0u8 ; 9] ; buffer [.. 8] . copy_from_slice (& (len as u64) . to_be_bytes ()) ; let offset = buffer . iter () . position (| i | * i != 0) . unwrap_or (7) ; buffer [8] = 8 - offset as u8 ; EncodedLen { offset , buffer } }
};
}
