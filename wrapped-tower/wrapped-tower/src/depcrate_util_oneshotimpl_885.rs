// Generated macro for impl_885 (impl)
macro_rules! Depcrate_util_oneshotimpl_885 {
() => {
// Module: crate::util::oneshot
// Provides: {"impl_885"}
// Dependencies: {}
impl < S , Req > Oneshot < S , Req > where S : Service < Req > , { # [allow (missing_docs)] pub const fn new (svc : S , req : Req) -> Self { Oneshot { state : State :: not_ready (svc , Some (req)) , } } }
};
}
