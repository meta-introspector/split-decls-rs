// Generated macro for impl_840 (impl)
macro_rules! Depcrate_cors_max_ageimpl_840 {
() => {
// Module: crate::cors::max_age
// Provides: {"impl_840"}
// Dependencies: {}
impl MaxAge { # [doc = " Set a static max-age value"] # [doc = ""] # [doc = " See [`CorsLayer::max_age`][super::CorsLayer::max_age] for more details."] pub fn exact (max_age : Duration) -> Self { Self (MaxAgeInner :: Exact (Some (max_age . as_secs () . into ()))) } # [doc = " Set the max-age based on the preflight request parts"] # [doc = ""] # [doc = " See [`CorsLayer::max_age`][super::CorsLayer::max_age] for more details."] pub fn dynamic < F > (f : F) -> Self where F : Fn (& HeaderValue , & RequestParts) -> Duration + Send + Sync + 'static , { Self (MaxAgeInner :: Fn (Arc :: new (f))) } pub (super) fn to_header (& self , origin : Option < & HeaderValue > , parts : & RequestParts ,) -> Option < (HeaderName , HeaderValue) > { let max_age = match & self . 0 { MaxAgeInner :: Exact (v) => v . clone () ? , MaxAgeInner :: Fn (c) => c (origin ? , parts) . as_secs () . into () , } ; Some ((header :: ACCESS_CONTROL_MAX_AGE , max_age)) } }
};
}
