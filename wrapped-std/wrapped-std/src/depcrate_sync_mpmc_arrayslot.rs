// Generated macro for Slot (struct)
macro_rules! Depcrate_sync_mpmc_arraySlot {
() => {
// Module: crate::sync::mpmc::array
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a channel."] struct Slot < T > { # [doc = " The current stamp."] stamp : Atomic < usize > , # [doc = " The message in this slot. Either read out in `read` or dropped through"] # [doc = " `discard_all_messages`."] msg : UnsafeCell < MaybeUninit < T > > , }
};
}
