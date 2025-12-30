// Generated macro for impl_585 (impl)
macro_rules! Depcrate_errorimpl_585 {
() => {
// Module: crate::error
// Provides: {"impl_585"}
// Dependencies: {}
# [unstable (feature = "error_reporter" , issue = "90172")] impl < E > fmt :: Display for Report < E > where E : Error , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . pretty { self . fmt_multiline (f) } else { self . fmt_singleline (f) } } }
};
}
