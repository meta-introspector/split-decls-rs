// Generated macro for DGRAM_PREFIX (const)
macro_rules! Depcrate_buf_factoryDGRAM_PREFIX {
() => {
// Module: crate::buf_factory
// Provides: {"DGRAM_PREFIX"}
// Dependencies: {}
# [doc = " Prefix size to reserve in a [`PooledDgram`]. Up to 8 bytes for the flow ID"] # [doc = " plus 1 byte for the flow context."] const DGRAM_PREFIX : usize = 8 + 1 ;
};
}
