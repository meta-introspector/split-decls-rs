// Generated macro for impl_54 (impl)
macro_rules! Depcrate_arcimpl_54 {
() => {
// Module: crate::arc
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Debug > fmt :: Debug for Arc < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
