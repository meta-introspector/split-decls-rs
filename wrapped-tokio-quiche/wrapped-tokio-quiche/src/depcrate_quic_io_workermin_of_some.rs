// Generated macro for min_of_some (function)
macro_rules! Depcrate_quic_io_workermin_of_some {
() => {
// Module: crate::quic::io::worker
// Provides: {"min_of_some"}
// Dependencies: {}
# [doc = " Returns the minimum of `v1` and `v2`, ignoring `None`s."] fn min_of_some < T : Ord > (v1 : Option < T > , v2 : Option < T >) -> Option < T > { match (v1 , v2) { (Some (a) , Some (b)) => Some (a . min (b)) , (Some (v) , _) | (_ , Some (v)) => Some (v) , (None , None) => None , } }
};
}
