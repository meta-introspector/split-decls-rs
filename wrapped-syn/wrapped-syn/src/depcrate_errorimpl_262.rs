// Generated macro for impl_262 (impl)
macro_rules! Depcrate_errorimpl_262 {
() => {
// Module: crate::error
// Provides: {"impl_262"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (& self . messages [0] . message) } }
};
}
