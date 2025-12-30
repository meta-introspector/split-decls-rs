// Generated macro for impl_53 (impl)
macro_rules! Depcrate_arcimpl_53 {
() => {
// Module: crate::arc
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Display > fmt :: Display for Arc < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
