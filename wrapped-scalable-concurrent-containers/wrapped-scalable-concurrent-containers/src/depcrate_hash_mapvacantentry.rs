// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_hash_mapVacantEntry {
() => {
// Module: crate::hash_map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " [`VacantEntry`] is a view into a vacant entry in a [`HashMap`]."] pub struct VacantEntry < 'h , K , V , H = RandomState > where H : BuildHasher , { hashmap : & 'h HashMap < K , V , H > , key : K , hash : u64 , locked_bucket : LockedBucket < K , V , () , MAP > , }
};
}
