// Generated macro for impl_234 (impl)
macro_rules! Depcrate_lineimpl_234 {
() => {
// Module: crate::line
// Provides: {"impl_234"}
// Dependencies: {}
impl LineSegmenterBorrowed < 'static > { # [doc = " Cheaply converts a [`LineSegmenterBorrowed<'static>`] into a [`LineSegmenter`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`LineSegmenter`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`LineSegmenterBorrowed`]."] pub fn static_to_owned (self) -> LineSegmenter { LineSegmenter { payload : DataPayload :: from_static_ref (self . data) , complex : self . complex . static_to_owned () , options : self . options , } } }
};
}
