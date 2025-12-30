// Generated macro for impl_24 (impl)
macro_rules! Depcrate_poolimpl_24 {
() => {
// Module: crate::pool
// Provides: {"impl_24"}
// Dependencies: {}
impl < T , C > fmt :: Debug for Ref < '_ , T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
};
}
