// Generated macro for impl_1279 (impl)
macro_rules! Depcrate_validate_requestimpl_1279 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1279"}
// Dependencies: {}
impl < ResBody > AcceptHeader < ResBody > { # [doc = " Create a new `AcceptHeader`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `header_value` is not in the form: `type/subtype`, such as `application/json`"] fn new (header_value : & str) -> Self where ResBody : Default , { Self { header_value : Arc :: new (header_value . parse :: < Mime > () . expect ("value is not a valid header value") ,) , _ty : PhantomData , } } }
};
}
