// Generated macro for impl_18 (impl)
macro_rules! Depcrate_request_spanimpl_18 {
() => {
// Module: crate::request_span
// Provides: {"impl_18"}
// Dependencies: {}
impl < S , R , G > Service < S , R , G > where S : tower_service :: Service < R > , G : GetSpan < R > + Clone , { pub fn new (inner : S , get_span : G) -> Self { Service { get_span , inner , _p : PhantomData , } } }
};
}
