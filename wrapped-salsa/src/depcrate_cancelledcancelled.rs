// Generated macro for Cancelled (enum)
macro_rules! Depcrate_cancelledCancelled {
() => {
// Module: crate::cancelled
// Provides: {"Cancelled"}
// Dependencies: {}
# [doc = " A panic payload indicating that execution of a salsa query was cancelled."] # [doc = ""] # [doc = " This can occur for a few reasons:"] # [doc = " *"] # [doc = " *"] # [doc = " *"] # [derive (Debug)] # [non_exhaustive] pub enum Cancelled { # [doc = " The query was operating on revision R, but there is a pending write to move to revision R+1."] # [non_exhaustive] PendingWrite , # [doc = " The query was blocked on another thread, and that thread panicked."] # [non_exhaustive] PropagatedPanic , }
};
}
