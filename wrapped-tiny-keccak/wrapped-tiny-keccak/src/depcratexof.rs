// Generated macro for Xof (trait)
macro_rules! DepcrateXof {
() => {
// Module: crate
// Provides: {"Xof"}
// Dependencies: {}
# [doc = " Extendable-output function (`XOF`) is a function on bit strings in which the output can be"] # [doc = " extended to any desired length."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::Xof;"] # [doc = " #"] # [doc = " # fn foo<X: Xof>(mut xof: X) {"] # [doc = " let mut output = [0u8; 64];"] # [doc = " xof.squeeze(&mut output[0..32]);"] # [doc = " xof.squeeze(&mut output[32..]);"] # [doc = " # }"] # [doc = " ```"] pub trait Xof { # [doc = " A method used to retrieve another part of hash function output."] fn squeeze (& mut self , output : & mut [u8]) ; }
};
}
