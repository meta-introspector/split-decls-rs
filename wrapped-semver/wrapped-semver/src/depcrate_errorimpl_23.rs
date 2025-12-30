// Generated macro for impl_23 (impl)
macro_rules! Depcrate_errorimpl_23 {
() => {
// Module: crate::error
// Provides: {"impl_23"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("Error(\"") ? ; Display :: fmt (self , formatter) ? ; formatter . write_str ("\")") ? ; Ok (()) } }
};
}
