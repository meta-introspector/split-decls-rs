// Generated macro for DATAGRAM_POOL (static)
macro_rules! Depcrate_buf_factoryDATAGRAM_POOL {
() => {
// Module: crate::buf_factory
// Provides: {"DATAGRAM_POOL"}
// Dependencies: {}
# [doc = " A datagram pool shared for both UDP streams, and incoming QUIC packets."] static DATAGRAM_POOL : BufPool = BufPool :: new (DATAGRAM_POOL_SIZE , MAX_DATAGRAM_SIZE) ;
};
}
