// Generated macro for ReachabilityError (enum)
macro_rules! Depcrate_network_reachabilityReachabilityError {
() => {
// Module: crate::network_reachability
// Provides: {"ReachabilityError"}
// Dependencies: {}
# [doc = " Failure to determine reachability"] # [derive (Debug)] pub enum ReachabilityError { # [doc = " `SCNetworkReachabilityGetFlags` call failed."] FailedToDetermineReachability , # [doc = "  `SCNetworkReachabilityGetFlags` call returned unrecognized flags."] UnrecognizedFlags (u32) , }
};
}
