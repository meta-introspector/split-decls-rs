// Generated macro for impl_20 (impl)
macro_rules! Depcrate_poolimpl_20 {
() => {
// Module: crate::pool
// Provides: {"impl_20"}
// Dependencies: {}
impl < T , C > fmt :: Debug for Pool < T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pool") . field ("shards" , & self . shards) . field ("config" , & C :: debug ()) . finish () } }
};
}
