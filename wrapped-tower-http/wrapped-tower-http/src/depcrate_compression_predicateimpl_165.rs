// Generated macro for impl_165 (impl)
macro_rules! Depcrate_compression_predicateimpl_165 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_165"}
// Dependencies: {}
impl SizeAbove { pub (crate) const DEFAULT_MIN_SIZE : u16 = 32 ; # [doc = " Create a new `SizeAbove` predicate that will only compress responses larger than"] # [doc = " `min_size_bytes`."] # [doc = ""] # [doc = " The response will be compressed if the exact size cannot be determined through either the"] # [doc = " `content-length` header or [`Body::size_hint`]."] pub const fn new (min_size_bytes : u16) -> Self { Self (min_size_bytes) } }
};
}
