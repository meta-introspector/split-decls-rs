// Generated macro for impl_971 (impl)
macro_rules! Depcrate_builderimpl_971 {
() => {
// Module: crate::builder
// Provides: {"impl_971"}
// Dependencies: {}
impl < L : fmt :: Debug > fmt :: Debug for ServiceBuilder < L > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("ServiceBuilder") . field (& self . layer) . finish () } }
};
}
