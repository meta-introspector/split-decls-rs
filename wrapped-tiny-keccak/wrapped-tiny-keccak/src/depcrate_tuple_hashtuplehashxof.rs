// Generated macro for TupleHashXof (struct)
macro_rules! Depcrate_tuple_hashTupleHashXof {
() => {
// Module: crate::tuple_hash
// Provides: {"TupleHashXof"}
// Dependencies: {}
# [doc = " The `TupleHashXOF` extendable-output functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"tuple_hash\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{TupleHash, Xof, IntoXof, Hasher};"] # [doc = " let input = b\"hello world\";"] # [doc = " let mut output = [0u8; 64];"] # [doc = " let mut hasher = TupleHash::v256(b\"\");"] # [doc = " hasher.update(input);"] # [doc = " let mut xof = hasher.into_xof();"] # [doc = " xof.squeeze(&mut output[..32]);"] # [doc = " xof.squeeze(&mut output[32..]);"] # [doc = " ```"] # [doc = ""] # [doc = " ---"] # [doc = ""] # [doc = " [`TupleHashXof`] can be created only by using [`TupleHash::IntoXof`] interface."] # [doc = ""] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [doc = " [`TupleHashXof`]: struct.TupleHashXof.html"] # [doc = " [`TupleHash::IntoXof`]: struct.TupleHash.html#impl-IntoXof"] # [derive (Clone)] pub struct TupleHashXof { state : CShake , }
};
}
