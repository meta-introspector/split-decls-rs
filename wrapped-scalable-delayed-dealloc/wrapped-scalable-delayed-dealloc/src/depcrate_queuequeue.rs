// Generated macro for Queue (struct)
macro_rules! Depcrate_queueQueue {
() => {
// Module: crate::queue
// Provides: {"Queue"}
// Dependencies: {}
# [doc = " [`Queue`] is a lock-free concurrent first-in-first-out container."] pub struct Queue < T > { # [doc = " `oldest` points to the oldest entry in the [`Queue`]."] oldest : AtomicShared < LinkedEntry < T > > , # [doc = " `newest` *eventually* points to the newest entry in the [`Queue`]."] newest : AtomicShared < LinkedEntry < T > > , }
};
}
