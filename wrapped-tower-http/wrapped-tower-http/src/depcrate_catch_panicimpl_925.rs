// Generated macro for impl_925 (impl)
macro_rules! Depcrate_catch_panicimpl_925 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_925"}
// Dependencies: {}
impl < S > CatchPanic < S , DefaultResponseForPanic > { # [doc = " Create a new `CatchPanic` with the default panic handler."] pub fn new (inner : S) -> Self { Self { inner , panic_handler : DefaultResponseForPanic , } } }
};
}
