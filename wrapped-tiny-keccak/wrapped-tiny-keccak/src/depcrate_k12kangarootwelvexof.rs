// Generated macro for KangarooTwelveXof (struct)
macro_rules! Depcrate_k12KangarooTwelveXof {
() => {
// Module: crate::k12
// Provides: {"KangarooTwelveXof"}
// Dependencies: {}
# [doc = " The `KangarooTwelve` extendable-output function defined [`here`]."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"k12\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{KangarooTwelve, Xof, IntoXof, Hasher};"] # [doc = " let input = b\"hello world\";"] # [doc = " let mut output = [0u8; 64];"] # [doc = " let mut hasher = KangarooTwelve::new(b\"\");"] # [doc = " hasher.update(input);"] # [doc = " let mut xof = hasher.into_xof();"] # [doc = " xof.squeeze(&mut output[..32]);"] # [doc = " xof.squeeze(&mut output[32..]);"] # [doc = " ```"] # [doc = ""] # [doc = " ---"] # [doc = ""] # [doc = " [`KangarooTwelveXof`] can be created only by using [`KangarooTwelve::IntoXof`] interface."] # [doc = ""] # [doc = " [`here`]: https://eprint.iacr.org/2016/770.pdf"] # [doc = " [`KangarooTwelveXof`]: struct.KangarooTwelveXof.html"] # [doc = " [`KangarooTwelve::IntoXof`]: struct.KangarooTwelve.html#impl-IntoXof"] # [derive (Clone)] pub struct KangarooTwelveXof { state : KeccakState < KeccakP > , }
};
}
