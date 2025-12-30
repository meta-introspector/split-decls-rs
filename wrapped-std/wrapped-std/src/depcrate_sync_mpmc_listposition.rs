// Generated macro for Position (struct)
macro_rules! Depcrate_sync_mpmc_listPosition {
() => {
// Module: crate::sync::mpmc::list
// Provides: {"Position"}
// Dependencies: {}
# [doc = " A position in a channel."] # [derive (Debug)] struct Position < T > { # [doc = " The index in the channel."] index : Atomic < usize > , # [doc = " The block in the linked list."] block : Atomic < * mut Block < T > > , }
};
}
