// Generated macro for impl_357 (impl)
macro_rules! Depcrate_decompression_bodyimpl_357 {
() => {
// Module: crate::decompression::body
// Provides: {"impl_357"}
// Dependencies: {}
# [cfg (feature = "decompression-zstd")] impl < B > DecorateAsyncRead for ZstdDecoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = ZstdDecoder < Self :: Input > ; fn apply (input : Self :: Input , _quality : CompressionLevel) -> Self :: Output { let mut decoder = ZstdDecoder :: new (input) ; decoder . multiple_members (true) ; decoder } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
