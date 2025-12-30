// Generated macro for byte (function)
macro_rules! Depcrate_helpers_stringbyte {
() => {
// Module: crate::helpers::string
// Provides: {"byte"}
// Dependencies: {}
fn byte (s : impl AsRef < [u8] > , idx : usize) -> u8 { s . as_ref () . get (idx) . copied () . unwrap_or_default () }
};
}
