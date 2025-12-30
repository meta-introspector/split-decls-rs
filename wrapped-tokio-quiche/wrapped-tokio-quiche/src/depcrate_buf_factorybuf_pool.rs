// Generated macro for BUF_POOL (static)
macro_rules! Depcrate_buf_factoryBUF_POOL {
() => {
// Module: crate::buf_factory
// Provides: {"BUF_POOL"}
// Dependencies: {}
# [doc = " A generic buffer pool used to pass data around without copying."] static BUF_POOL : BufPool = BufPool :: new (POOL_SIZE , MAX_POOL_BUF_SIZE) ;
};
}
