// Generated macro for Reserve (struct)
macro_rules! Depcrate_hash_mapReserve {
() => {
// Module: crate::hash_map
// Provides: {"Reserve"}
// Dependencies: {}
# [doc = " [`Reserve`] keeps the capacity of the associated [`HashMap`] higher than a certain level."] # [doc = ""] # [doc = " The [`HashMap`] does not shrink the capacity below the reserved capacity."] pub struct Reserve < 'h , K , V , H = RandomState > where K : Eq + Hash , H : BuildHasher , { hashmap : & 'h HashMap < K , V , H > , additional : usize , }
};
}
