// Generated macro for impl_19 (impl)
macro_rules! Depcrate_block_apiimpl_19 {
() => {
// Module: crate::block_api
// Provides: {"impl_19"}
// Dependencies: {}
impl < const V2 : bool > AlgorithmName for TigerCore < V2 > { # [inline] fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if V2 { f . write_str ("Tiger2") } else { f . write_str ("Tiger") } } }
};
}
