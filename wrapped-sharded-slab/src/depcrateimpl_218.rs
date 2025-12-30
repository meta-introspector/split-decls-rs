// Generated macro for impl_218 (impl)
macro_rules! Depcrateimpl_218 {
() => {
// Module: crate
// Provides: {"impl_218"}
// Dependencies: {}
impl < T : fmt :: Debug , C : cfg :: Config > fmt :: Debug for Slab < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Slab") . field ("shards" , & self . shards) . field ("config" , & C :: debug ()) . finish () } }
};
}
