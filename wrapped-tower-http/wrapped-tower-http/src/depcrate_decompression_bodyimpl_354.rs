// Generated macro for impl_354 (impl)
macro_rules! Depcrate_decompression_bodyimpl_354 {
() => {
// Module: crate::decompression::body
// Provides: {"impl_354"}
// Dependencies: {}
# [cfg (feature = "decompression-gzip")] impl < B > DecorateAsyncRead for GzipDecoder < B > where B : Body , { type Input = AsyncReadBody < B > ; type Output = GzipDecoder < Self :: Input > ; fn apply (input : Self :: Input , _quality : CompressionLevel) -> Self :: Output { let mut decoder = GzipDecoder :: new (input) ; decoder . multiple_members (true) ; decoder } fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > { pinned . get_pin_mut () } }
};
}
