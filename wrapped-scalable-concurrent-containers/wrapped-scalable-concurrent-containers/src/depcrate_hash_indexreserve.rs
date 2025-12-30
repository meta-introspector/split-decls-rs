// Generated macro for Reserve (struct)
macro_rules! Depcrate_hash_indexReserve {
() => {
// Module: crate::hash_index
// Provides: {"Reserve"}
// Dependencies: {}
# [doc = " [`Reserve`] keeps the capacity of the associated [`HashIndex`] higher than a certain level."] # [doc = ""] # [doc = " The [`HashIndex`] does not shrink the capacity below the reserved capacity."] pub struct Reserve < 'h , K , V , H = RandomState > where K : Eq + Hash , H : BuildHasher , { hashindex : & 'h HashIndex < K , V , H > , additional : usize , }
};
}
