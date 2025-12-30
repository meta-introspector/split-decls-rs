// Generated macro for TupleHash (struct)
macro_rules! Depcrate_tuple_hashTupleHash {
() => {
// Module: crate::tuple_hash
// Provides: {"TupleHash"}
// Dependencies: {}
# [doc = " The `TupleHash` hash functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " `TupleHash` is designed to provide a generic, misuse-resistant way to combine a sequence of"] # [doc = " strings for hashing such that, for example, a `TupleHash` computed on the tuple (`\"abc\"` ,`\"d\"`) will"] # [doc = " produce a different hash value than a `TupleHash` computed on the tuple (`\"ab\"`,`\"cd\"`), even though"] # [doc = " all the remaining input parameters are kept the same, and the two resulting concatenated"] # [doc = " strings, without string encoding, are identical."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"tuple_hash\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [derive (Clone)] pub struct TupleHash { state : CShake , }
};
}
