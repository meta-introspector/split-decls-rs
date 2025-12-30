// Generated macro for Rng (trait)
macro_rules! Depcrate_util_rngRng {
() => {
// Module: crate::util::rng
// Provides: {"Rng"}
// Dependencies: {}
# [doc = " A simple [PRNG] trait for use within tower middleware."] # [doc = ""] # [doc = " [PRNG]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator"] pub trait Rng { # [doc = " Generate a random [`u64`]."] fn next_u64 (& mut self) -> u64 ; # [doc = " Generate a random [`f64`] between `[0, 1)`."] fn next_f64 (& mut self) -> f64 { let float_size = std :: mem :: size_of :: < f64 > () as u32 * 8 ; let precision = 52 + 1 ; let scale = 1.0 / ((1u64 << precision) as f64) ; let value = self . next_u64 () ; let value = value >> (float_size - precision) ; scale * value as f64 } # [doc = " Randomly pick a value within the range."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " - If `range.start >= range.end` this will panic in debug mode."] fn next_range (& mut self , range : Range < u64 >) -> u64 { debug_assert ! (range . start < range . end , "The range start must be smaller than the end") ; let start = range . start ; let end = range . end ; let range = end - start ; let n = self . next_u64 () ; (n % range) + start } }
};
}
