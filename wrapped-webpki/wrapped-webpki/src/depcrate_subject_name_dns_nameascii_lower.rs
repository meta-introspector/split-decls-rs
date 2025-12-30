// Generated macro for ascii_lower (function)
macro_rules! Depcrate_subject_name_dns_nameascii_lower {
() => {
// Module: crate::subject_name::dns_name
// Provides: {"ascii_lower"}
// Dependencies: {}
# [inline] fn ascii_lower (b : u8) -> u8 { match b { b'A' ..= b'Z' => b + b'a' - b'A' , _ => b , } }
};
}
