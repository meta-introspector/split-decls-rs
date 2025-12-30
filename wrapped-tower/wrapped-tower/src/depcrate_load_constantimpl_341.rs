// Generated macro for impl_341 (impl)
macro_rules! Depcrate_load_constantimpl_341 {
() => {
// Module: crate::load::constant
// Provides: {"impl_341"}
// Dependencies: {}
impl < T , M : Copy > Constant < T , M > { # [doc = " Wraps a `T`-typed service with a constant `M`-typed load metric."] pub const fn new (inner : T , load : M) -> Self { Self { inner , load } } }
};
}
