// Generated macro for mark_scan_enforced (function)
macro_rules! Depcrate_collectormark_scan_enforced {
() => {
// Module: crate::collector
// Provides: {"mark_scan_enforced"}
// Dependencies: {}
# [doc = " Marks the head of a chain to indicate that there is a potentially unreachable `Collector` in the"] # [doc = " chain."] fn mark_scan_enforced () { let _result = GLOBAL_ROOT . chain_head . fetch_update (Release , Relaxed , | p | { let new_tag = match Tag :: into_tag (p) { Tag :: None => Tag :: Second , Tag :: First => Tag :: Both , Tag :: Second | Tag :: Both => return None , } ; Some (Tag :: update_tag (p , new_tag) . cast_mut ()) }) ; }
};
}
