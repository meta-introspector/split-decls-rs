// Generated macro for impl_30 (impl)
macro_rules! Depcrate_poolimpl_30 {
() => {
// Module: crate::pool
// Provides: {"impl_30"}
// Dependencies: {}
impl < T , C > fmt :: Debug for RefMut < '_ , T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
};
}
