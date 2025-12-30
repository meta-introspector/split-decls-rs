// Generated macro for impl_161 (impl)
macro_rules! Depcrate_compression_predicateimpl_161 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_161"}
// Dependencies: {}
impl DefaultPredicate { # [doc = " Create a new `DefaultPredicate`."] pub fn new () -> Self { let inner = SizeAbove :: new (SizeAbove :: DEFAULT_MIN_SIZE) . and (NotForContentType :: GRPC) . and (NotForContentType :: IMAGES) . and (NotForContentType :: SSE) ; Self (inner) } }
};
}
