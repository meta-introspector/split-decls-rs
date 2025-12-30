// Generated macro for impl_118 (impl)
macro_rules! Depcrate_dispatcherimpl_118 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_118"}
// Dependencies: {}
impl < S > From < S > for Dispatch where S : Subscriber + Send + Sync + 'static , { # [inline] fn from (subscriber : S) -> Self { Dispatch :: new (subscriber) } }
};
}
