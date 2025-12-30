// Generated macro for impl_231 (impl)
macro_rules! Depcrate_collectorimpl_231 {
() => {
// Module: crate::collector
// Provides: {"impl_231"}
// Dependencies: {}
impl Drop for CollectorAnchor { # [inline] fn drop (& mut self) { unsafe { LOCAL_COLLECTOR . with (| local_collector | { let collector_ptr = local_collector . load (Relaxed) ; if ! collector_ptr . is_null () { (* collector_ptr) . state . fetch_or (Collector :: INVALID , Release) ; } let mut temp_collector = Collector :: default () ; temp_collector . state . store (Collector :: INACTIVE , Relaxed) ; local_collector . store (addr_of_mut ! (temp_collector) , Release) ; if ! Collector :: clear_chain () { mark_scan_enforced () ; } Collector :: clear_for_drop (addr_of_mut ! (temp_collector)) ; local_collector . store (ptr :: null_mut () , Release) ; }) ; } } }
};
}
