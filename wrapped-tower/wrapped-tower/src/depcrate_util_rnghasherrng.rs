// Generated macro for HasherRng (struct)
macro_rules! Depcrate_util_rngHasherRng {
() => {
// Module: crate::util::rng
// Provides: {"HasherRng"}
// Dependencies: {}
# [doc = " A [`Rng`] implementation that uses a [`Hasher`] to generate the random"] # [doc = " values. The implementation uses an internal counter to pass to the hasher"] # [doc = " for each iteration of [`Rng::next_u64`]."] # [doc = ""] # [doc = " # Default"] # [doc = ""] # [doc = " This hasher has a default type of [`RandomState`] which just uses the"] # [doc = " libstd method of getting a random u64."] # [derive (Clone , Debug)] pub struct HasherRng < H = RandomState > { hasher : H , counter : u64 , }
};
}
