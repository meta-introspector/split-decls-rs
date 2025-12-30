// Generated macro for Channel (struct)
macro_rules! Depcrate_sync_mpmc_arrayChannel {
() => {
// Module: crate::sync::mpmc::array
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " Bounded channel based on a preallocated array."] pub (crate) struct Channel < T > { # [doc = " The head of the channel."] # [doc = ""] # [doc = " This value is a \"stamp\" consisting of an index into the buffer, a mark bit, and a lap, but"] # [doc = " packed into a single `usize`. The lower bits represent the index, while the upper bits"] # [doc = " represent the lap. The mark bit in the head is always zero."] # [doc = ""] # [doc = " Messages are popped from the head of the channel."] head : CachePadded < Atomic < usize > > , # [doc = " The tail of the channel."] # [doc = ""] # [doc = " This value is a \"stamp\" consisting of an index into the buffer, a mark bit, and a lap, but"] # [doc = " packed into a single `usize`. The lower bits represent the index, while the upper bits"] # [doc = " represent the lap. The mark bit indicates that the channel is disconnected."] # [doc = ""] # [doc = " Messages are pushed into the tail of the channel."] tail : CachePadded < Atomic < usize > > , # [doc = " The buffer holding slots."] buffer : Box < [Slot < T >] > , # [doc = " The channel capacity."] cap : usize , # [doc = " A stamp with the value of `{ lap: 1, mark: 0, index: 0 }`."] one_lap : usize , # [doc = " If this bit is set in the tail, that means the channel is disconnected."] mark_bit : usize , # [doc = " Senders waiting while the channel is full."] senders : SyncWaker , # [doc = " Receivers waiting while the channel is empty and not disconnected."] receivers : SyncWaker , }
};
}
