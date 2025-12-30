// Generated macro for impl_154 (impl)
macro_rules! Depcrate_common_networkimpl_154 {
() => {
// Module: crate::common::network
// Provides: {"impl_154"}
// Dependencies: {}
impl fmt :: Display for IpNetwork { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}/{}" , self . addr , self . prefix) } }
};
}
