// Generated macro for impl_258 (impl)
macro_rules! Depcrate_errorimpl_258 {
() => {
// Module: crate::error
// Provides: {"impl_258"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { if self . messages . len () == 1 { formatter . debug_tuple ("Error") . field (& self . messages [0]) . finish () } else { formatter . debug_tuple ("Error") . field (& self . messages) . finish () } } }
};
}
