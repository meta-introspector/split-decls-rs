// Generated macro for test_rng (function)
macro_rules! Depcrate_test_helperstest_rng {
() => {
// Module: crate::test_helpers
// Provides: {"test_rng"}
// Dependencies: {}
# [doc = " Test-only replacement for `rand::thread_rng()`, which is unusable for"] # [doc = " us, as we want to allow running stdlib tests on tier-3 targets which may"] # [doc = " not have `getrandom` support."] # [doc = ""] # [doc = " Does a bit of a song and dance to ensure that the seed is different on"] # [doc = " each call (as some tests sadly rely on this), but doesn't try that hard."] # [doc = ""] # [doc = " This is duplicated in the `core`, `alloc` test suites (as well as"] # [doc = " `std`'s integration tests), but figuring out a mechanism to share these"] # [doc = " seems far more painful than copy-pasting a 7 line function a couple"] # [doc = " times, given that even under a perma-unstable feature, I don't think we"] # [doc = " want to expose types from `rand` from `std`."] # [track_caller] pub (crate) fn test_rng () -> rand_xorshift :: XorShiftRng { let mut hasher = RandomState :: new () . build_hasher () ; Location :: caller () . hash (& mut hasher) ; let hc64 = hasher . finish () ; let seed_vec = hc64 . to_le_bytes () . into_iter () . chain (0u8 .. 8) . collect :: < Vec < u8 > > () ; let seed : [u8 ; 16] = seed_vec . as_slice () . try_into () . unwrap () ; SeedableRng :: from_seed (seed) }
};
}
