// Generated macro for impl_43 (impl)
macro_rules! Depcrate_poolimpl_43 {
() => {
// Module: crate::pool
// Provides: {"impl_43"}
// Dependencies: {}
impl < T , C > fmt :: Debug for OwnedRefMut < T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
};
}
