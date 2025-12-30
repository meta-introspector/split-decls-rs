// Generated macro for impl_154 (impl)
macro_rules! Depcrate_graphemeimpl_154 {
() => {
// Module: crate::grapheme
// Provides: {"impl_154"}
// Dependencies: {}
impl GraphemeClusterSegmenterBorrowed < 'static > { # [doc = " Cheaply converts a [`GraphemeClusterSegmenterBorrowed<'static>`] into a [`GraphemeClusterSegmenter`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`GraphemeClusterSegmenter`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`GraphemeClusterSegmenterBorrowed`]."] pub const fn static_to_owned (self) -> GraphemeClusterSegmenter { GraphemeClusterSegmenter { payload : DataPayload :: from_static_ref (self . data) , } } }
};
}
