// Generated macro for impl_951 (impl)
macro_rules! Depcrate_util_rngimpl_951 {
() => {
// Module: crate::util::rng
// Provides: {"impl_951"}
// Dependencies: {}
impl < H > Rng for HasherRng < H > where H : BuildHasher , { fn next_u64 (& mut self) -> u64 { let mut hasher = self . hasher . build_hasher () ; hasher . write_u64 (self . counter) ; self . counter = self . counter . wrapping_add (1) ; hasher . finish () } }
};
}
