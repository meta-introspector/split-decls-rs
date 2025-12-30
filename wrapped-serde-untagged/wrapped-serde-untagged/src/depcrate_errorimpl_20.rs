// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorimpl_20 {
() => {
// Module: crate::error
// Provides: {"impl_20"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let error = self . as_serde :: < serde :: de :: value :: Error > () ; Display :: fmt (& error , formatter) } }
};
}
