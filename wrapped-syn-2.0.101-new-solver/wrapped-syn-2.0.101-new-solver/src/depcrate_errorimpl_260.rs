// Generated macro for impl_260 (impl)
macro_rules! Depcrate_errorimpl_260 {
() => {
// Module: crate::error
// Provides: {"impl_260"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (& self . messages [0] . message) } }
};
}
