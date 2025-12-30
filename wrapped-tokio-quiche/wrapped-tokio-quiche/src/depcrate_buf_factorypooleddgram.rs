// Generated macro for PooledDgram (type)
macro_rules! Depcrate_buf_factoryPooledDgram {
() => {
// Module: crate::buf_factory
// Provides: {"PooledDgram"}
// Dependencies: {}
# [doc = " A pooled byte buffer to pass datagrams around without copying."] # [doc = ""] # [doc = " The buffer type records a head offset, which allows cheaply inserting"] # [doc = " data at the front given sufficient capacity."] pub type PooledDgram = Pooled < ConsumeBuffer > ;
};
}
