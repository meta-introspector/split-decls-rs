// Generated macro for impl_79 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_79 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_79"}
// Dependencies: {}
impl < F > From < F > for FilterFn < F > where F : Fn (& Metadata < '_ >) -> bool , { fn from (enabled : F) -> Self { Self :: new (enabled) } }
};
}
