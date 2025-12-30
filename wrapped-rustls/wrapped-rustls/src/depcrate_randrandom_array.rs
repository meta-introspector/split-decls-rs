// Generated macro for random_array (function)
macro_rules! Depcrate_randrandom_array {
() => {
// Module: crate::rand
// Provides: {"random_array"}
// Dependencies: {}
# [doc = " Make an array of size `N` containing random material."] pub (crate) fn random_array < const N : usize > (secure_random : & dyn SecureRandom ,) -> Result < [u8 ; N] , GetRandomFailed > { let mut v = [0 ; N] ; secure_random . fill (& mut v) ? ; Ok (v) }
};
}
