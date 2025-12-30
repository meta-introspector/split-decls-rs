// Generated macro for clamp_capacity (function)
macro_rules! Depcrate_streamclamp_capacity {
() => {
// Module: crate::stream
// Provides: {"clamp_capacity"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [inline] pub (crate) fn clamp_capacity < T > (capacity : usize) -> usize { # [doc = " Don't pre-allocate more than 64KiB when calling `Vec::with_capacity`."] # [doc = ""] # [doc = " Pre-allocating memory is a nice optimization but count fields can't"] # [doc = " always be trusted. We should clamp initial capacities to some reasonable"] # [doc = " amount. This reduces the risk of a bogus count value triggering a panic"] # [doc = " due to an OOM error."] # [doc = ""] # [doc = " This does not affect correctness. `winnow` will always read the full number"] # [doc = " of elements regardless of the capacity cap."] const MAX_INITIAL_CAPACITY_BYTES : usize = 65536 ; let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES / core :: mem :: size_of :: < T > () . max (1) ; capacity . min (max_initial_capacity) }
};
}
