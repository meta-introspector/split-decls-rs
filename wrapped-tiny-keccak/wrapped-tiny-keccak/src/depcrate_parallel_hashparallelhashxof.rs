// Generated macro for ParallelHashXof (struct)
macro_rules! Depcrate_parallel_hashParallelHashXof {
() => {
// Module: crate::parallel_hash
// Provides: {"ParallelHashXof"}
// Dependencies: {}
# [doc = " The `ParallelHashXOF` extendable-output functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"parallel_hash\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{ParallelHash, Xof, IntoXof, Hasher};"] # [doc = " let input = b\"hello world\";"] # [doc = " let mut output = [0u8; 64];"] # [doc = " let mut hasher = ParallelHash::v256(b\"\", 8);"] # [doc = " hasher.update(input);"] # [doc = " let mut xof = hasher.into_xof();"] # [doc = " xof.squeeze(&mut output[..32]);"] # [doc = " xof.squeeze(&mut output[32..]);"] # [doc = " ```"] # [doc = ""] # [doc = " ---"] # [doc = ""] # [doc = " [`ParallelHashXof`] can be created only by using [`ParallelHash::IntoXof`] interface."] # [doc = ""] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [doc = " [`ParallelHashXof`]: struct.ParallelHashXof.html"] # [doc = " [`ParallelHash::IntoXof`]: struct.ParallelHash.html#impl-IntoXof"] # [derive (Clone)] pub struct ParallelHashXof { state : CShake , }
};
}
