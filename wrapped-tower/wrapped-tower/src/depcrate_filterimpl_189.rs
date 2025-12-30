// Generated macro for impl_189 (impl)
macro_rules! Depcrate_filterimpl_189 {
() => {
// Module: crate::filter
// Provides: {"impl_189"}
// Dependencies: {}
impl < T , U > AsyncFilter < T , U > { # [doc = " Returns a new [`AsyncFilter`] service wrapping `inner`."] pub const fn new (inner : T , predicate : U) -> Self { Self { inner , predicate } } # [doc = " Returns a new [`Layer`] that wraps services with an [`AsyncFilter`]"] # [doc = " service with the given [`AsyncPredicate`]."] # [doc = ""] # [doc = " [`Layer`]: crate::Layer"] pub fn layer (predicate : U) -> FilterLayer < U > { FilterLayer :: new (predicate) } # [doc = " Check a `Request` value against this filter's predicate."] pub async fn check < R > (& mut self , request : R) -> Result < U :: Request , BoxError > where U : AsyncPredicate < R > , { self . predicate . check (request) . await } # [doc = " Get a reference to the inner service"] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to the inner service"] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Consume `self`, returning the inner service"] pub fn into_inner (self) -> T { self . inner } }
};
}
