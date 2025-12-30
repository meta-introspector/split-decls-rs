// Generated macro for Bag (struct)
macro_rules! Depcrate_bagBag {
() => {
// Module: crate::bag
// Provides: {"Bag"}
// Dependencies: {}
# [doc = " [`Bag`] is a lock-free concurrent unordered instance container."] # [doc = ""] # [doc = " [`Bag`] is a linearizable concurrent instance container where `ARRAY_LEN` instances are stored"] # [doc = " in a fixed-size array, and the rest are managed by its backup container; this makes a [`Bag`]"] # [doc = " especially efficient if the expected number of instances does not exceed `ARRAY_LEN`."] # [doc = ""] # [doc = " The maximum value of `ARRAY_LEN` is limited to `usize::BITS / 2` which is the default value."] # [derive (Debug)] pub struct Bag < T , const ARRAY_LEN : usize = DEFAULT_ARRAY_LEN > { # [doc = " Primary storage."] primary_storage : Storage < T , ARRAY_LEN > , # [doc = " Fallback storage."] stack : Stack < Storage < T , ARRAY_LEN > > , }
};
}
