// Generated macro for impl_87 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_87 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_87"}
// Dependencies: {}
impl < F , S > From < F > for DynFilterFn < S , F > where F : Fn (& Metadata < '_ > , & Context < '_ , S >) -> bool , { fn from (f : F) -> Self { Self :: new (f) } }
};
}
