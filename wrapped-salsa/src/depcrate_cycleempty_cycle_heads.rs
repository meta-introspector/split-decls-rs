// Generated macro for empty_cycle_heads (function)
macro_rules! Depcrate_cycleempty_cycle_heads {
() => {
// Module: crate::cycle
// Provides: {"empty_cycle_heads"}
// Dependencies: {}
# [inline] pub (crate) fn empty_cycle_heads () -> & 'static CycleHeads { static EMPTY_CYCLE_HEADS : OnceLock < CycleHeads > = OnceLock :: new () ; EMPTY_CYCLE_HEADS . get_or_init (| | CycleHeads (ThinVec :: new ())) }
};
}
