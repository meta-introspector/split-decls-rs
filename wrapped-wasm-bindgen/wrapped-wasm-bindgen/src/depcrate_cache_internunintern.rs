// Generated macro for unintern (function)
macro_rules! Depcrate_cache_internunintern {
() => {
// Module: crate::cache::intern
// Provides: {"unintern"}
// Dependencies: {}
# [doc = " Removes a Rust string from the intern cache."] # [doc = ""] # [doc = " This does the opposite of the [`intern`](fn.intern.html) function."] # [doc = ""] # [doc = " If the [`intern`](fn.intern.html) function is called again then it will re-intern the string."] # [allow (unused_variables)] # [inline] pub fn unintern (s : & str) { # [cfg (feature = "enable-interning")] unintern_str (s) ; }
};
}
