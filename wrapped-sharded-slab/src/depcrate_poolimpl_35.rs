// Generated macro for impl_35 (impl)
macro_rules! Depcrate_poolimpl_35 {
() => {
// Module: crate::pool
// Provides: {"impl_35"}
// Dependencies: {}
impl < T , C > fmt :: Debug for OwnedRef < T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
};
}
