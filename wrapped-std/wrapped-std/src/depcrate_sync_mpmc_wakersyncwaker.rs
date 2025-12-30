// Generated macro for SyncWaker (struct)
macro_rules! Depcrate_sync_mpmc_wakerSyncWaker {
() => {
// Module: crate::sync::mpmc::waker
// Provides: {"SyncWaker"}
// Dependencies: {}
# [doc = " A waker that can be shared among threads without locking."] # [doc = ""] # [doc = " This is a simple wrapper around `Waker` that internally uses a mutex for synchronization."] pub (crate) struct SyncWaker { # [doc = " The inner `Waker`."] inner : Mutex < Waker > , # [doc = " `true` if the waker is empty."] is_empty : Atomic < bool > , }
};
}
