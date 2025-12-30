// Generated macro for impl_42 (impl)
macro_rules! Depcrate_network_reachabilityimpl_42 {
() => {
// Module: crate::network_reachability
// Provides: {"impl_42"}
// Dependencies: {}
impl Display for ReachabilityError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: FailedToDetermineReachability => write ! (f , "Failed to determine reachability") , Self :: UnrecognizedFlags (flags) => { write ! (f , "Unrecognized reachability flags: {}" , flags) } } } }
};
}
