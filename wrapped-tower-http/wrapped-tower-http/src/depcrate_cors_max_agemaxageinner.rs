// Generated macro for MaxAgeInner (enum)
macro_rules! Depcrate_cors_max_ageMaxAgeInner {
() => {
// Module: crate::cors::max_age
// Provides: {"MaxAgeInner"}
// Dependencies: {}
# [derive (Clone)] enum MaxAgeInner { Exact (Option < HeaderValue >) , Fn (Arc < dyn for < 'a > Fn (& 'a HeaderValue , & 'a RequestParts) -> Duration + Send + Sync + 'static >) , }
};
}
