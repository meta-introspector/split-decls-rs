// Generated macro for OriginInner (enum)
macro_rules! Depcrate_cors_allow_originOriginInner {
() => {
// Module: crate::cors::allow_origin
// Provides: {"OriginInner"}
// Dependencies: {}
# [derive (Clone)] enum OriginInner { Const (HeaderValue) , List (Vec < HeaderValue >) , Predicate (Arc < dyn for < 'a > Fn (& 'a HeaderValue , & 'a RequestParts) -> bool + Send + Sync + 'static > ,) , AsyncPredicate (Arc < dyn for < 'a > Fn (HeaderValue , & 'a RequestParts ,) -> Pin < Box < dyn Future < Output = bool > + Send + 'static > > + Send + Sync + 'static , > ,) , }
};
}
