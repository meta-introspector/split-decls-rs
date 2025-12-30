// Generated macro for impl_199 (impl)
macro_rules! Depcrate_compression_bodyimpl_199 {
() => {
// Module: crate::compression::body
// Provides: {"impl_199"}
// Dependencies: {}
# [cfg (feature = "compression-gzip")] impl < B > DecorateAsyncRead for GzipEncoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = GzipEncoder < Self :: Input > ; fn apply (input : Self :: Input , quality : CompressionLevel) -> Self :: Output { GzipEncoder :: with_quality (input , quality . into_async_compression ()) } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
