// Generated macro for impl_19 (impl)
macro_rules! Depcrate_asciiimpl_19 {
() => {
// Module: crate::ascii
// Provides: {"impl_19"}
// Dependencies: {}
impl < const N : usize > fmt :: Display for TinyAsciiStr < N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
};
}
