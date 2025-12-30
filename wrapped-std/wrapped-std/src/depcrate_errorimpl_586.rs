// Generated macro for impl_586 (impl)
macro_rules! Depcrate_errorimpl_586 {
() => {
// Module: crate::error
// Provides: {"impl_586"}
// Dependencies: {}
# [unstable (feature = "error_reporter" , issue = "90172")] impl < E > fmt :: Debug for Report < E > where Report < E > : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
};
}
