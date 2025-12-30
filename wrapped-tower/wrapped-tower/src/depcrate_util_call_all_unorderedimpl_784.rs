// Generated macro for impl_784 (impl)
macro_rules! Depcrate_util_call_all_unorderedimpl_784 {
() => {
// Module: crate::util::call_all::unordered
// Provides: {"impl_784"}
// Dependencies: {}
impl < Svc , S > CallAllUnordered < Svc , S > where Svc : Service < S :: Item > , S : Stream , { # [doc = " Create new [`CallAllUnordered`] combinator."] # [doc = ""] # [doc = " [`Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html"] pub fn new (service : Svc , stream : S) -> CallAllUnordered < Svc , S > { CallAllUnordered { inner : common :: CallAll :: new (service , stream , FuturesUnordered :: new ()) , } } # [doc = " Extract the wrapped [`Service`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if [`take_service`] was already called."] # [doc = ""] # [doc = " [`take_service`]: crate::util::CallAllUnordered::take_service"] pub fn into_inner (self) -> Svc { self . inner . into_inner () } # [doc = " Extract the wrapped `Service`."] # [doc = ""] # [doc = " This [`CallAllUnordered`] can no longer be used after this function has been called."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if [`take_service`] was already called."] # [doc = ""] # [doc = " [`take_service`]: crate::util::CallAllUnordered::take_service"] pub fn take_service (self : Pin < & mut Self >) -> Svc { self . project () . inner . take_service () } }
};
}
