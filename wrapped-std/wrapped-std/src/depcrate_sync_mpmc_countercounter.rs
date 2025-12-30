// Generated macro for Counter (struct)
macro_rules! Depcrate_sync_mpmc_counterCounter {
() => {
// Module: crate::sync::mpmc::counter
// Provides: {"Counter"}
// Dependencies: {}
# [doc = " Reference counter internals."] struct Counter < C > { # [doc = " The number of senders associated with the channel."] senders : Atomic < usize > , # [doc = " The number of receivers associated with the channel."] receivers : Atomic < usize > , # [doc = " Set to `true` if the last sender or the last receiver reference deallocates the channel."] destroy : Atomic < bool > , # [doc = " The internal channel."] chan : C , }
};
}
