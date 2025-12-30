// Generated macro for impl_922 (impl)
macro_rules! Depcrate_catch_panicimpl_922 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_922"}
// Dependencies: {}
impl < T > CatchPanicLayer < T > { # [doc = " Create a new `CatchPanicLayer` with a custom panic handler."] pub fn custom (panic_handler : T) -> Self where T : ResponseForPanic , { Self { panic_handler } } }
};
}
