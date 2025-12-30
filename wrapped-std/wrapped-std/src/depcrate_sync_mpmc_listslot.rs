// Generated macro for Slot (struct)
macro_rules! Depcrate_sync_mpmc_listSlot {
() => {
// Module: crate::sync::mpmc::list
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a block."] struct Slot < T > { # [doc = " The message."] msg : UnsafeCell < MaybeUninit < T > > , # [doc = " The state of the slot."] state : Atomic < usize > , }
};
}
