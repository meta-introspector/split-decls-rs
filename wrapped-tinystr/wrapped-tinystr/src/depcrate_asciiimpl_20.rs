// Generated macro for impl_20 (impl)
macro_rules! Depcrate_asciiimpl_20 {
() => {
// Module: crate::ascii
// Provides: {"impl_20"}
// Dependencies: {}
impl < const N : usize > Deref for TinyAsciiStr < N > { type Target = str ; # [inline] fn deref (& self) -> & str { self . as_str () } }
};
}
