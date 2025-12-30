// Generated macro for isgraph (function)
macro_rules! Depcrate_deflateisgraph {
() => {
// Module: crate::deflate
// Provides: {"isgraph"}
// Dependencies: {}
# [doc = " Checks that symbol is a printing character (excluding space)"] # [allow (unused)] fn isgraph (c : u8) -> bool { (c > 0x20) && (c <= 0x7E) }
};
}
