// Generated macro for impl_355 (impl)
macro_rules! Depcrate_decompression_bodyimpl_355 {
() => {
// Module: crate::decompression::body
// Provides: {"impl_355"}
// Dependencies: {}
# [cfg (feature = "decompression-deflate")] impl < B > DecorateAsyncRead for ZlibDecoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = ZlibDecoder < Self :: Input > ; fn apply (input : Self :: Input , _quality : CompressionLevel) -> Self :: Output { ZlibDecoder :: new (input) } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
