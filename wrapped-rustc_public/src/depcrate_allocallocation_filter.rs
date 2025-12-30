// Generated macro for allocation_filter (function)
macro_rules! Depcrate_allocallocation_filter {
() => {
// Module: crate::alloc
// Provides: {"allocation_filter"}
// Dependencies: {}
# [doc = " Creates an `Allocation` only from information within the `AllocRange`."] pub (super) fn allocation_filter < 'tcx > (alloc : & rustc_middle :: mir :: interpret :: Allocation , alloc_range : AllocRange , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { alloc :: allocation_filter (alloc , alloc_range , tables , cx) }
};
}
