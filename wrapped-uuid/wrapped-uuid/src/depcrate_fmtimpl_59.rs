// Generated macro for impl_59 (impl)
macro_rules! Depcrate_fmtimpl_59 {
() => {
// Module: crate::fmt
// Provides: {"impl_59"}
// Dependencies: {}
impl fmt :: LowerHex for Uuid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self . as_hyphenated () , f) } }
};
}
