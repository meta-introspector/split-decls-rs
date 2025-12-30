// Generated macro for Block (struct)
macro_rules! Depcrate_sync_mpmc_listBlock {
() => {
// Module: crate::sync::mpmc::list
// Provides: {"Block"}
// Dependencies: {}
# [doc = " A block in a linked list."] # [doc = ""] # [doc = " Each block in the list can hold up to `BLOCK_CAP` messages."] struct Block < T > { # [doc = " The next block in the linked list."] next : Atomic < * mut Block < T > > , # [doc = " Slots for messages."] slots : [Slot < T > ; BLOCK_CAP] , }
};
}
