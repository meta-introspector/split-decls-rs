// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_hash_indexVacantEntry {
() => {
// Module: crate::hash_index
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " [`VacantEntry`] is a view into a vacant entry in a [`HashIndex`]."] pub struct VacantEntry < 'h , K , V , H = RandomState > where H : BuildHasher , { hashindex : & 'h HashIndex < K , V , H > , key : K , hash : u64 , locked_bucket : LockedBucket < K , V , () , INDEX > , }
};
}
