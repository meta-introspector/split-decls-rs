// Generated macro for rust_const_name (function)
macro_rules! Depcrate_writerrust_const_name {
() => {
// Module: crate::writer
// Provides: {"rust_const_name"}
// Dependencies: {}
# [doc = " Heuristically produce an appropriate constant Rust name."] fn rust_const_name (s : & str) -> String { let mut s = s . replace ('.' , "_") . to_string () ; s . make_ascii_uppercase () ; s }
};
}
