// Generated macro for impl_293 (impl)
macro_rules! Depcrate_wordimpl_293 {
() => {
// Module: crate::word
// Provides: {"impl_293"}
// Dependencies: {}
impl WordSegmenterBorrowed < 'static > { # [doc = " Cheaply converts a [`WordSegmenterBorrowed<'static>`] into a [`WordSegmenter`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`WordSegmenter`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`WordSegmenterBorrowed`]."] pub fn static_to_owned (self) -> WordSegmenter { let payload_locale_override = self . locale_override . map (DataPayload :: from_static_ref) ; WordSegmenter { payload : DataPayload :: from_static_ref (self . data) , complex : self . complex . static_to_owned () , payload_locale_override , } } }
};
}
