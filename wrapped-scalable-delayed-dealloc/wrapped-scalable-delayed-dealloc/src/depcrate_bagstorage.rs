// Generated macro for Storage (struct)
macro_rules! Depcrate_bagStorage {
() => {
// Module: crate::bag
// Provides: {"Storage"}
// Dependencies: {}
# [derive (Debug)] struct Storage < T , const ARRAY_LEN : usize > { # [doc = " Storage."] storage : UnsafeCell < [MaybeUninit < T > ; ARRAY_LEN] > , # [doc = " Storage metadata."] # [doc = ""] # [doc = " The layout of the metadata is,"] # [doc = " - Upper `usize::BITS / 2` bits: initialization bitmap."] # [doc = " - Lower `usize::BITS / 2` bits: owned state bitmap."] # [doc = ""] # [doc = " The metadata represents four possible states of a storage slot."] # [doc = " - `!instantiated && !owned`: initial state."] # [doc = " - `!instantiated && owned`: owned for instantiating."] # [doc = " - `instantiated && !owned`: valid and reachable."] # [doc = " - `instantiated && owned`: owned for moving out the instance."] metadata : AtomicUsize , }
};
}
