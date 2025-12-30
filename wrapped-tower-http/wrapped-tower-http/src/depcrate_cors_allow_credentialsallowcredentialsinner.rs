// Generated macro for AllowCredentialsInner (enum)
macro_rules! Depcrate_cors_allow_credentialsAllowCredentialsInner {
() => {
// Module: crate::cors::allow_credentials
// Provides: {"AllowCredentialsInner"}
// Dependencies: {}
# [derive (Clone)] enum AllowCredentialsInner { Yes , No , Predicate (Arc < dyn for < 'a > Fn (& 'a HeaderValue , & 'a RequestParts) -> bool + Send + Sync + 'static > ,) , }
};
}
