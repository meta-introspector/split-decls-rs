// Generated macro for macro_881 (macro)
macro_rules! Depcrate_util_oneshotmacro_881 {
() => {
// Module: crate::util::oneshot
// Provides: {"macro_881"}
// Dependencies: {}
pin_project ! { # [doc = " A [`Future`] consuming a [`Service`] and request, waiting until the [`Service`]"] # [doc = " is ready, and then calling [`Service::call`] with the request, and"] # [doc = " waiting for that [`Future`]."] # [derive (Debug)] pub struct Oneshot < S : Service < Req >, Req > { # [pin] state : State < S , Req >, } }
};
}
