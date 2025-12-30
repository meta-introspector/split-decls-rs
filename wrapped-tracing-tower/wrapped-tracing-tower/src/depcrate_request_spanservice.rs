// Generated macro for Service (struct)
macro_rules! Depcrate_request_spanService {
() => {
// Module: crate::request_span
// Provides: {"Service"}
// Dependencies: {}
# [derive (Debug)] pub struct Service < S , R , G = fn (& R) -> tracing :: Span > where S : tower_service :: Service < R > , G : GetSpan < R > , { get_span : G , inner : S , _p : PhantomData < fn (R) > , }
};
}
