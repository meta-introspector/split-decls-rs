// Generated macro for Size (trait)
macro_rules! DepcrateSize {
() => {
// Module: crate
// Provides: {"Size"}
// Dependencies: {}
# [doc = " The `Size` trait needs to be implemented by any struct that should be"] # [doc = " efficiently serialized."] # [doc = " This allows to collect the length of a serialized structure before allocating"] # [doc = " memory."] pub trait Size { fn tls_serialized_len (& self) -> usize ; }
};
}
