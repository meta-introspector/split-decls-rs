// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorimpl_27 {
() => {
// Module: crate::error
// Provides: {"impl_27"}
// Dependencies: {}
impl < E > Display for TracedError < E > where E : std :: error :: Error , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Display :: fmt (& self . inner . error , f) } }
};
}
