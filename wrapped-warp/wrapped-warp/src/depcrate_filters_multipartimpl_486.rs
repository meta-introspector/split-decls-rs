// Generated macro for impl_486 (impl)
macro_rules! Depcrate_filters_multipartimpl_486 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_486"}
// Dependencies: {}
impl FormOptions { # [doc = " Set the maximum byte length allowed for this body."] # [doc = ""] # [doc = " `max_length(None)` means that maximum byte length is not checked."] # [doc = " Defaults to 2MB."] pub fn max_length (mut self , max : impl Into < Option < u64 > >) -> Self { self . max_length = max . into () ; self } }
};
}
