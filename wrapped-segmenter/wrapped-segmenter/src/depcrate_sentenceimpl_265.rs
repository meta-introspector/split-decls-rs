// Generated macro for impl_265 (impl)
macro_rules! Depcrate_sentenceimpl_265 {
() => {
// Module: crate::sentence
// Provides: {"impl_265"}
// Dependencies: {}
impl SentenceSegmenterBorrowed < 'static > { # [doc = " Cheaply converts a [`SentenceSegmenterBorrowed<'static>`] into a [`SentenceSegmenter`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`SentenceSegmenter`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`SentenceSegmenterBorrowed`]."] pub const fn static_to_owned (self) -> SentenceSegmenter { let payload_locale_override = if let Some (d) = self . locale_override { Some (DataPayload :: from_static_ref (d)) } else { None } ; SentenceSegmenter { payload : DataPayload :: from_static_ref (self . data) , payload_locale_override , } } }
};
}
