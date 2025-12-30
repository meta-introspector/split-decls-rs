// Generated macro for size_of_val (function)
macro_rules! Depcratesize_of_val {
() => {
// Module: crate
// Provides: {"size_of_val"}
// Dependencies: {}
# [doc = " Returns the size of the pointed-to value in bytes."] # [doc = ""] # [doc = " This is a `const fn` in the standard library starting in Rust 1.85. When MSRV is at least that,"] # [doc = " this can be removed."] # [inline] const fn size_of_val < T > (_ : & T) -> usize { size_of :: < T > () }
};
}
