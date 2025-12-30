// Generated macro for PendingRequests (struct)
macro_rules! Depcrate_load_pending_requestsPendingRequests {
() => {
// Module: crate::load::pending_requests
// Provides: {"PendingRequests"}
// Dependencies: {}
# [doc = " Measures the load of the underlying service using the number of currently-pending requests."] # [derive (Debug)] pub struct PendingRequests < S , C = CompleteOnResponse > { service : S , ref_count : RefCount , completion : C , }
};
}
