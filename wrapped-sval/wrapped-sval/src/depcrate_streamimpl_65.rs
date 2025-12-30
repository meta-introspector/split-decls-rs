// Generated macro for impl_65 (impl)
macro_rules! Depcrate_streamimpl_65 {
() => {
// Module: crate::stream
// Provides: {"impl_65"}
// Dependencies: {}
impl < S : ? Sized > Computed < S > { # [inline] fn new_borrowed < 'a > (stream : & 'a mut S) -> & 'a mut Computed < S > { unsafe { & mut * (stream as * mut _ as * mut Computed < S >) } } }
};
}
