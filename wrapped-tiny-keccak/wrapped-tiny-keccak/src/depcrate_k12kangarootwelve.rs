// Generated macro for KangarooTwelve (struct)
macro_rules! Depcrate_k12KangarooTwelve {
() => {
// Module: crate::k12
// Provides: {"KangarooTwelve"}
// Dependencies: {}
# [doc = " The `KangarooTwelve` hash function defined [`here`]."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"k12\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " [`here`]: https://eprint.iacr.org/2016/770.pdf"] # [derive (Clone)] pub struct KangarooTwelve < T > { state : KeccakState < KeccakP > , current_chunk : KeccakState < KeccakP > , custom_string : Option < T > , written : usize , chunks : usize , }
};
}
