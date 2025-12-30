// Generated macro for Sha3 (struct)
macro_rules! Depcrate_sha3Sha3 {
() => {
// Module: crate::sha3
// Provides: {"Sha3"}
// Dependencies: {}
# [doc = " The `SHA3` hash functions defined in [`FIPS-202`]."] # [doc = ""] # [doc = " [`FIPS-202`]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf"] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"sha3\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{Hasher, Sha3};"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " let input = b\"hello world\";"] # [doc = " let mut output = [0; 32];"] # [doc = " let expected = b\"\\"] # [doc = "     \\x64\\x4b\\xcc\\x7e\\x56\\x43\\x73\\x04\\x09\\x99\\xaa\\xc8\\x9e\\x76\\x22\\xf3\\"] # [doc = "     \\xca\\x71\\xfb\\xa1\\xd9\\x72\\xfd\\x94\\xa3\\x1c\\x3b\\xfb\\xf2\\x4e\\x39\\x38\\"] # [doc = " \";"] # [doc = " let mut sha3 = Sha3::v256();"] # [doc = " sha3.update(input);"] # [doc = " sha3.finalize(&mut output);"] # [doc = " assert_eq!(expected, &output);"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone)] pub struct Sha3 { state : KeccakState < KeccakF > , }
};
}
