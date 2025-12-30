// Generated macro for is_ascii_lowercase_hex (function)
macro_rules! Depcrate_normalizeis_ascii_lowercase_hex {
() => {
// Module: crate::normalize
// Provides: {"is_ascii_lowercase_hex"}
// Dependencies: {}
fn is_ascii_lowercase_hex (s : & str) -> bool { s . bytes () . all (| b | matches ! (b , b'0' ..= b'9' | b'a' ..= b'f')) }
};
}
