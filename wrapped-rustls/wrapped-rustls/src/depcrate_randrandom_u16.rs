// Generated macro for random_u16 (function)
macro_rules! Depcrate_randrandom_u16 {
() => {
// Module: crate::rand
// Provides: {"random_u16"}
// Dependencies: {}
# [doc = " Return a uniformly random [`u16`]."] pub (crate) fn random_u16 (secure_random : & dyn SecureRandom) -> Result < u16 , GetRandomFailed > { Ok (u16 :: from_be_bytes (random_array (secure_random) ?)) }
};
}
