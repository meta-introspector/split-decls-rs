// Generated macro for impl_1269 (impl)
macro_rules! Depcrate_validate_requestimpl_1269 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1269"}
// Dependencies: {}
impl < S , ResBody > ValidateRequestHeader < S , AcceptHeader < ResBody > > { # [doc = " Validate requests have the required Accept header."] # [doc = ""] # [doc = " The `Accept` header is required to be `*/*`, `type/*` or `type/subtype`,"] # [doc = " as configured."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " See `AcceptHeader::new` for when this method panics."] pub fn accept (inner : S , value : & str) -> Self where ResBody : Default , { Self :: custom (inner , AcceptHeader :: new (value)) } }
};
}
