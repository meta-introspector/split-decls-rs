// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errorimpl_26 {
() => {
// Module: crate::error
// Provides: {"impl_26"}
// Dependencies: {}
impl < E > Debug for TracedError < E > where E : std :: error :: Error , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Debug :: fmt (& self . inner . error , f) } }
};
}
