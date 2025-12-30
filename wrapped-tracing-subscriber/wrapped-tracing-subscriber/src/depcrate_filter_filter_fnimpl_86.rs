// Generated macro for impl_86 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_86 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_86"}
// Dependencies: {}
impl < S , F , R > Clone for DynFilterFn < S , F , R > where F : Clone , R : Clone , { fn clone (& self) -> Self { Self { enabled : self . enabled . clone () , register_callsite : self . register_callsite . clone () , max_level_hint : self . max_level_hint , _s : PhantomData , } } }
};
}
