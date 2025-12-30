// Generated macro for Inner (struct)
macro_rules! Depcrate_sync_mpmc_zeroInner {
() => {
// Module: crate::sync::mpmc::zero
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Inner representation of a zero-capacity channel."] struct Inner { # [doc = " Senders waiting to pair up with a receive operation."] senders : Waker , # [doc = " Receivers waiting to pair up with a send operation."] receivers : Waker , # [doc = " Equals `true` when the channel is disconnected."] is_disconnected : bool , }
};
}
