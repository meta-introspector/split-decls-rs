// Generated macro for impl_20 (impl)
macro_rules! Depcrate_block_apiimpl_20 {
() => {
// Module: crate::block_api
// Provides: {"impl_20"}
// Dependencies: {}
impl < const V2 : bool > fmt :: Debug for TigerCore < V2 > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if V2 { f . write_str ("Tiger2Core { ... }") } else { f . write_str ("TigerCore { ... }") } } }
};
}
