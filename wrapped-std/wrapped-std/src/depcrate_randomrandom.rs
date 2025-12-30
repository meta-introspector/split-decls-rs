// Generated macro for random (function)
macro_rules! Depcrate_randomrandom {
() => {
// Module: crate::random
// Provides: {"random"}
// Dependencies: {}
# [doc = " Generates a random value from a distribution, using the default random source."] # [doc = ""] # [doc = " This is a convenience function for `dist.sample(&mut DefaultRandomSource)` and will sample"] # [doc = " according to the same distribution as the underlying [`Distribution`] trait implementation. See"] # [doc = " [`DefaultRandomSource`] for more information about how randomness is sourced."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Generating a [version 4/variant 1 UUID] represented as text:"] # [doc = " ```"] # [doc = " #![feature(random)]"] # [doc = ""] # [doc = " use std::random::random;"] # [doc = ""] # [doc = " let bits: u128 = random(..);"] # [doc = " let g1 = (bits >> 96) as u32;"] # [doc = " let g2 = (bits >> 80) as u16;"] # [doc = " let g3 = (0x4000 | (bits >> 64) & 0x0fff) as u16;"] # [doc = " let g4 = (0x8000 | (bits >> 48) & 0x3fff) as u16;"] # [doc = " let g5 = (bits & 0xffffffffffff) as u64;"] # [doc = " let uuid = format!(\"{g1:08x}-{g2:04x}-{g3:04x}-{g4:04x}-{g5:012x}\");"] # [doc = " println!(\"{uuid}\");"] # [doc = " ```"] # [doc = ""] # [doc = " [version 4/variant 1 UUID]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random)"] # [unstable (feature = "random" , issue = "130703")] pub fn random < T > (dist : impl Distribution < T >) -> T { dist . sample (& mut DefaultRandomSource) }
};
}
