// Generated macro for impl_200 (impl)
macro_rules! Depcrate_compression_bodyimpl_200 {
() => {
// Module: crate::compression::body
// Provides: {"impl_200"}
// Dependencies: {}
# [cfg (feature = "compression-deflate")] impl < B > DecorateAsyncRead for ZlibEncoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = ZlibEncoder < Self :: Input > ; fn apply (input : Self :: Input , quality : CompressionLevel) -> Self :: Output { ZlibEncoder :: with_quality (input , quality . into_async_compression ()) } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
