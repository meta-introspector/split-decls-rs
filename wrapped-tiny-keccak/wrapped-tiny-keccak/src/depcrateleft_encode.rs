// Generated macro for left_encode (function)
macro_rules! Depcrateleft_encode {
() => {
// Module: crate
// Provides: {"left_encode"}
// Dependencies: {}
fn left_encode (len : usize) -> EncodedLen { let mut buffer = [0u8 ; 9] ; buffer [1 ..] . copy_from_slice (& (len as u64) . to_be_bytes ()) ; let offset = buffer . iter () . position (| i | * i != 0) . unwrap_or (8) ; buffer [offset - 1] = 9 - offset as u8 ; EncodedLen { offset : offset - 1 , buffer , } }
};
}
