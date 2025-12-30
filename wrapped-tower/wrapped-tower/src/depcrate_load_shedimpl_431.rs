// Generated macro for impl_431 (impl)
macro_rules! Depcrate_load_shedimpl_431 {
() => {
// Module: crate::load_shed
// Provides: {"impl_431"}
// Dependencies: {}
impl < S > LoadShed < S > { # [doc = " Wraps a service in [`LoadShed`] middleware."] pub const fn new (inner : S) -> Self { LoadShed { inner , is_ready : false , } } }
};
}
