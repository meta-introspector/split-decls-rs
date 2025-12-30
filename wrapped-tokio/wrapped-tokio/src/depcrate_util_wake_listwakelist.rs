// Generated macro for WakeList (struct)
macro_rules! Depcrate_util_wake_listWakeList {
() => {
// Module: crate::util::wake_list
// Provides: {"WakeList"}
// Dependencies: {}
# [doc = " A list of wakers to be woken."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " The first `curr` elements of `inner` are initialized."] pub (crate) struct WakeList { inner : [MaybeUninit < Waker > ; NUM_WAKERS] , curr : usize , }
};
}
