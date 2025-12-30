// Generated macro for Hasher (trait)
macro_rules! DepcrateHasher {
() => {
// Module: crate
// Provides: {"Hasher"}
// Dependencies: {}
# [doc = " A trait for hashing an arbitrary stream of bytes."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::Hasher;"] # [doc = " #"] # [doc = " # fn foo<H: Hasher>(mut hasher: H) {"] # [doc = " let input_a = b\"hello world\";"] # [doc = " let input_b = b\"!\";"] # [doc = " let mut output = [0u8; 32];"] # [doc = " hasher.update(input_a);"] # [doc = " hasher.update(input_b);"] # [doc = " hasher.finalize(&mut output);"] # [doc = " # }"] # [doc = " ```"] pub trait Hasher { # [doc = " Absorb additional input. Can be called multiple times."] fn update (& mut self , input : & [u8]) ; # [doc = " Pad and squeeze the state to the output."] fn finalize (self , output : & mut [u8]) ; }
};
}
