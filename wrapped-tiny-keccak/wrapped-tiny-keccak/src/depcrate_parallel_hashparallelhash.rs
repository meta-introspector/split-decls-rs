// Generated macro for ParallelHash (struct)
macro_rules! Depcrate_parallel_hashParallelHash {
() => {
// Module: crate::parallel_hash
// Provides: {"ParallelHash"}
// Dependencies: {}
# [doc = " The `ParallelHash` hash functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " The purpose of `ParallelHash` is to support the efficient hashing of very long strings, by"] # [doc = " taking advantage of the parallelism available in modern processors. `ParallelHash` supports the"] # [doc = " [`128-bit`] and [`256-bit`] security strengths, and also provides variable-length output."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"parallel_hash\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [doc = " [`128-bit`]: struct.ParallelHash.html#method.v128"] # [doc = " [`256-bit`]: struct.ParallelHash.html#method.v256"] # [derive (Clone)] pub struct ParallelHash { state : CShake , block_size : usize , bits : usize , blocks : usize , unfinished : Option < UnfinishedState > , }
};
}
