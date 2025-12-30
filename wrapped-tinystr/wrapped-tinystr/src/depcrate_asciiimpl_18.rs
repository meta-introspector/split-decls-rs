// Generated macro for impl_18 (impl)
macro_rules! Depcrate_asciiimpl_18 {
() => {
// Module: crate::ascii
// Provides: {"impl_18"}
// Dependencies: {}
impl < const N : usize > fmt :: Debug for TinyAsciiStr < N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
};
}
