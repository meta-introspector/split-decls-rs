// Generated macro for impl_926 (impl)
macro_rules! Depcrate_catch_panicimpl_926 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_926"}
// Dependencies: {}
impl < S , T > CatchPanic < S , T > { define_inner_service_accessors ! () ; # [doc = " Create a new `CatchPanic` with a custom panic handler."] pub fn custom (inner : S , panic_handler : T) -> Self where T : ResponseForPanic , { Self { inner , panic_handler , } } }
};
}
