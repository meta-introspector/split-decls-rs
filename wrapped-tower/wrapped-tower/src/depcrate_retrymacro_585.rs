// Generated macro for macro_585 (macro)
macro_rules! Depcrate_retrymacro_585 {
() => {
// Module: crate::retry
// Provides: {"macro_585"}
// Dependencies: {}
pin_project ! { # [doc = " Configure retrying requests of \"failed\" responses."] # [doc = ""] # [doc = " A [`Policy`] classifies what is a \"failed\" response."] # [doc = ""] # [doc = " # Clone"] # [doc = ""] # [doc = " This middleware requires that the inner `Service` implements [`Clone`],"] # [doc = " because the `Service` must be stored in each [`ResponseFuture`] in"] # [doc = " order to retry the request in the event of a failure. If the inner"] # [doc = " `Service` type does not implement `Clone`, the [`Buffer`] middleware"] # [doc = " can be added to make any `Service` cloneable."] # [doc = ""] # [doc = " [`Buffer`]: crate::buffer::Buffer"] # [doc = ""] # [doc = " The `Policy` must also implement `Clone`. This middleware will"] # [doc = " clone the policy for each _request session_. This means a new clone"] # [doc = " of the policy will be created for each initial request and any subsequent"] # [doc = " retries of that request. Therefore, any state stored in the `Policy` instance"] # [doc = " is for that request session only. In order to share data across request"] # [doc = " sessions, that shared state may be stored in an [`Arc`], so that all clones"] # [doc = " of the `Policy` type reference the same instance of the shared state."] # [doc = ""] # [doc = " [`Arc`]: std::sync::Arc"] # [derive (Clone , Debug)] pub struct Retry < P , S > { policy : P , service : S , } }
};
}
