// Generated macro for impl_239 (impl)
macro_rules! Depcrate_compression_serviceimpl_239 {
() => {
// Module: crate::compression::service
// Provides: {"impl_239"}
// Dependencies: {}
impl < S > Compression < S , DefaultPredicate > { # [doc = " Creates a new `Compression` wrapping the `service`."] pub fn new (service : S) -> Compression < S , DefaultPredicate > { Self { inner : service , accept : AcceptEncoding :: default () , predicate : DefaultPredicate :: default () , quality : CompressionLevel :: default () , } } }
};
}
