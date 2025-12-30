// Generated macro for impl_17 (impl)
macro_rules! Depcrate_request_spanimpl_17 {
() => {
// Module: crate::request_span
// Provides: {"impl_17"}
// Dependencies: {}
impl < S , R , G > Clone for Service < S , R , G > where S : tower_service :: Service < R > + Clone , G : GetSpan < R > + Clone , { fn clone (& self) -> Self { Service { get_span : self . get_span . clone () , inner : self . inner . clone () , _p : PhantomData , } } }
};
}
