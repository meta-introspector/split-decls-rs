// Generated macro for impl_356 (impl)
macro_rules! Depcrate_decompression_bodyimpl_356 {
() => {
// Module: crate::decompression::body
// Provides: {"impl_356"}
// Dependencies: {}
# [cfg (feature = "decompression-br")] impl < B > DecorateAsyncRead for BrotliDecoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = BrotliDecoder < Self :: Input > ; fn apply (input : Self :: Input , _quality : CompressionLevel) -> Self :: Output { BrotliDecoder :: new (input) } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
