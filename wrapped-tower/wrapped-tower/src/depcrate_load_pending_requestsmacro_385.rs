// Generated macro for macro_385 (macro)
macro_rules! Depcrate_load_pending_requestsmacro_385 {
() => {
// Module: crate::load::pending_requests
// Provides: {"macro_385"}
// Dependencies: {}
# [cfg (feature = "discover")] pin_project ! { # [doc = " Wraps a `D`-typed stream of discovered services with [`PendingRequests`]."] # [cfg_attr (docsrs , doc (cfg (feature = "discover")))] # [derive (Debug)] pub struct PendingRequestsDiscover < D , C = CompleteOnResponse > { # [pin] discover : D , completion : C , } }
};
}
