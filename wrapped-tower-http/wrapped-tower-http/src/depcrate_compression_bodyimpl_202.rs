// Generated macro for impl_202 (impl)
macro_rules! Depcrate_compression_bodyimpl_202 {
() => {
// Module: crate::compression::body
// Provides: {"impl_202"}
// Dependencies: {}
# [cfg (feature = "compression-zstd")] impl < B > DecorateAsyncRead for ZstdEncoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = ZstdEncoder < Self :: Input > ; fn apply (input : Self :: Input , quality : CompressionLevel) -> Self :: Output { let needs_window_limit = match quality { CompressionLevel :: Best => true , CompressionLevel :: Precise (level) => level >= 17 , _ => false , } ; if needs_window_limit { let params = [async_compression :: zstd :: CParameter :: window_log (23)] ; ZstdEncoder :: with_quality_and_params (input , quality . into_async_compression () , & params) } else { ZstdEncoder :: with_quality (input , quality . into_async_compression ()) } } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
