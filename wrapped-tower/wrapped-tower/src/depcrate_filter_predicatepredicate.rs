// Generated macro for Predicate (trait)
macro_rules! Depcrate_filter_predicatePredicate {
() => {
// Module: crate::filter::predicate
// Provides: {"Predicate"}
// Dependencies: {}
# [doc = " Checks a request synchronously."] pub trait Predicate < Request > { # [doc = " The type of requests returned by [`check`]."] # [doc = ""] # [doc = " This request is forwarded to the inner service if the predicate"] # [doc = " succeeds."] # [doc = ""] # [doc = " [`check`]: crate::filter::Predicate::check"] type Request ; # [doc = " Check whether the given request should be forwarded."] # [doc = ""] # [doc = " If the future resolves with [`Ok`], the request is forwarded to the inner service."] fn check (& mut self , request : Request) -> Result < Self :: Request , BoxError > ; }
};
}
