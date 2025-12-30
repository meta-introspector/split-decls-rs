// Generated macro for OnceLatch (struct)
macro_rules! Depcrate_latchOnceLatch {
() => {
// Module: crate::latch
// Provides: {"OnceLatch"}
// Dependencies: {}
# [doc = " Once latches are used to implement one-time blocking, primarily"] # [doc = " for the termination flag of the threads in the pool."] # [doc = ""] # [doc = " Note: like a `SpinLatch`, once-latches are always associated with"] # [doc = " some registry that is probing them, which must be tickled when"] # [doc = " they are set. *Unlike* a `SpinLatch`, they don't themselves hold a"] # [doc = " reference to that registry. This is because in some cases the"] # [doc = " registry owns the once-latch, and that would create a cycle. So a"] # [doc = " `OnceLatch` must be given a reference to its owning registry when"] # [doc = " it is set. For this reason, it does not implement the `Latch`"] # [doc = " trait (but it doesn't have to, as it is not used in those generic"] # [doc = " contexts)."] # [derive (Debug)] pub (super) struct OnceLatch { core_latch : CoreLatch , }
};
}
