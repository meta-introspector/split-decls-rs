// Generated macro for Hasher (struct)
macro_rules! Depcrate_xxhash32Hasher {
() => {
// Module: crate::xxhash32
// Provides: {"Hasher"}
// Dependencies: {}
# [doc = " Calculates the 32-bit hash."] # [doc = ""] # [doc = " ### Caution"] # [doc = ""] # [doc = " Although this struct implements [`hash::Hasher`][], it only calculates a"] # [doc = " 32-bit number, leaving the upper bits as 0. This means it is"] # [doc = " unlikely to be correct to use this in places like a [`HashMap`][std::collections::HashMap]."] # [derive (Debug , Clone , PartialEq)] pub struct Hasher { seed : u32 , accumulators : Accumulators , buffer : Buffer , length : u64 , }
};
}
