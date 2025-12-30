// Generated macro for impl_767 (impl)
macro_rules! Depcrate_cors_allow_credentialsimpl_767 {
() => {
// Module: crate::cors::allow_credentials
// Provides: {"impl_767"}
// Dependencies: {}
impl AllowCredentials { # [doc = " Allow credentials for all requests"] # [doc = ""] # [doc = " See [`CorsLayer::allow_credentials`] for more details."] # [doc = ""] # [doc = " [`CorsLayer::allow_credentials`]: super::CorsLayer::allow_credentials"] pub fn yes () -> Self { Self (AllowCredentialsInner :: Yes) } # [doc = " Allow credentials for some requests, based on a given predicate"] # [doc = ""] # [doc = " The first argument to the predicate is the request origin."] # [doc = ""] # [doc = " See [`CorsLayer::allow_credentials`] for more details."] # [doc = ""] # [doc = " [`CorsLayer::allow_credentials`]: super::CorsLayer::allow_credentials"] pub fn predicate < F > (f : F) -> Self where F : Fn (& HeaderValue , & RequestParts) -> bool + Send + Sync + 'static , { Self (AllowCredentialsInner :: Predicate (Arc :: new (f))) } pub (super) fn is_true (& self) -> bool { matches ! (& self . 0 , AllowCredentialsInner :: Yes) } pub (super) fn to_header (& self , origin : Option < & HeaderValue > , parts : & RequestParts ,) -> Option < (HeaderName , HeaderValue) > { # [allow (clippy :: declare_interior_mutable_const)] const TRUE : HeaderValue = HeaderValue :: from_static ("true") ; let allow_creds = match & self . 0 { AllowCredentialsInner :: Yes => true , AllowCredentialsInner :: No => false , AllowCredentialsInner :: Predicate (c) => c (origin ? , parts) , } ; allow_creds . then_some ((header :: ACCESS_CONTROL_ALLOW_CREDENTIALS , TRUE)) } }
};
}
