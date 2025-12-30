// Generated macro for impl_201 (impl)
macro_rules! Depcrate_compression_bodyimpl_201 {
() => {
// Module: crate::compression::body
// Provides: {"impl_201"}
// Dependencies: {}
# [cfg (feature = "compression-br")] impl < B > DecorateAsyncRead for BrotliEncoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = BrotliEncoder < Self :: Input > ; fn apply (input : Self :: Input , quality : CompressionLevel) -> Self :: Output { let level = match quality { CompressionLevel :: Default => async_compression :: Level :: Precise (4) , other => other . into_async_compression () , } ; BrotliEncoder :: with_quality (input , level) } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
