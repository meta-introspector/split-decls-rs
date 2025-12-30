// Generated macro for impl_180 (impl)
macro_rules! Depcrate_shardimpl_180 {
() => {
// Module: crate::shard
// Provides: {"impl_180"}
// Dependencies: {}
impl < T : fmt :: Debug , C : cfg :: Config > fmt :: Debug for Shard < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("Shard") ; # [cfg (debug_assertions)] d . field ("tid" , & self . tid) ; d . field ("shared" , & self . shared) . finish () } }
};
}
