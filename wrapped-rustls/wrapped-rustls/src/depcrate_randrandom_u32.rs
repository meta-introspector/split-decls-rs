// Generated macro for random_u32 (function)
macro_rules! Depcrate_randrandom_u32 {
() => {
// Module: crate::rand
// Provides: {"random_u32"}
// Dependencies: {}
# [doc = " Return a uniformly random [`u32`]."] pub (crate) fn random_u32 (secure_random : & dyn SecureRandom) -> Result < u32 , GetRandomFailed > { Ok (u32 :: from_be_bytes (random_array (secure_random) ?)) }
};
}
