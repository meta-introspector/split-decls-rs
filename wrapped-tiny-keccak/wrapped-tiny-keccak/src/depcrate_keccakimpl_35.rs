// Generated macro for impl_35 (impl)
macro_rules! Depcrate_keccakimpl_35 {
() => {
// Module: crate::keccak
// Provides: {"impl_35"}
// Dependencies: {}
impl Hasher for Keccak { # [doc = " Absorb additional input. Can be called multiple times."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{Hasher, Keccak};"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " # let mut keccak = Keccak::v256();"] # [doc = " keccak.update(b\"hello\");"] # [doc = " keccak.update(b\" world\");"] # [doc = " # }"] # [doc = " ```"] fn update (& mut self , input : & [u8]) { self . state . update (input) ; } # [doc = " Pad and squeeze the state to the output."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{Hasher, Keccak};"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " # let keccak = Keccak::v256();"] # [doc = " # let mut output = [0u8; 32];"] # [doc = " keccak.finalize(&mut output);"] # [doc = " # }"] # [doc = " #"] # [doc = " ```"] fn finalize (self , output : & mut [u8]) { self . state . finalize (output) ; } }
};
}
