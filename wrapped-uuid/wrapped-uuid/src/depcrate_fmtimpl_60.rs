// Generated macro for impl_60 (impl)
macro_rules! Depcrate_fmtimpl_60 {
() => {
// Module: crate::fmt
// Provides: {"impl_60"}
// Dependencies: {}
impl fmt :: UpperHex for Uuid { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self . as_hyphenated () , f) } }
};
}
