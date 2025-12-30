// Generated macro for NetworkReachabilityCallbackContext (struct)
macro_rules! Depcrate_network_reachabilityNetworkReachabilityCallbackContext {
() => {
// Module: crate::network_reachability
// Provides: {"NetworkReachabilityCallbackContext"}
// Dependencies: {}
struct NetworkReachabilityCallbackContext < T : Fn (ReachabilityFlags) + Sync + Send > { _host : SCNetworkReachability , callback : T , }
};
}
