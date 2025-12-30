// Generated macro for AsyncPredicate (trait)
macro_rules! Depcrate_filter_predicateAsyncPredicate {
() => {
// Module: crate::filter::predicate
// Provides: {"AsyncPredicate"}
// Dependencies: {}
# [doc = " Checks a request asynchronously."] pub trait AsyncPredicate < Request > { # [doc = " The future returned by [`check`]."] # [doc = ""] # [doc = " [`check`]: crate::filter::AsyncPredicate::check"] type Future : Future < Output = Result < Self :: Request , BoxError > > ; # [doc = " The type of requests returned by [`check`]."] # [doc = ""] # [doc = " This request is forwarded to the inner service if the predicate"] # [doc = " succeeds."] # [doc = ""] # [doc = " [`check`]: crate::filter::AsyncPredicate::check"] type Request ; # [doc = " Check whether the given request should be forwarded."] # [doc = ""] # [doc = " If the future resolves with [`Ok`], the request is forwarded to the inner service."] fn check (& mut self , request : Request) -> Self :: Future ; }
};
}
