// Generated macro for KmacXof (struct)
macro_rules! Depcrate_kmacKmacXof {
() => {
// Module: crate::kmac
// Provides: {"KmacXof"}
// Dependencies: {}
# [doc = " The `KMACXOF` extendable-output functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"kmac\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use tiny_keccak::{Kmac, Xof, IntoXof, Hasher};"] # [doc = " let input = b\"hello world\";"] # [doc = " let mut output = [0u8; 64];"] # [doc = " let mut kmac = Kmac::v256(b\"\", b\"\");"] # [doc = " kmac.update(input);"] # [doc = " let mut xof = kmac.into_xof();"] # [doc = " xof.squeeze(&mut output[..32]);"] # [doc = " xof.squeeze(&mut output[32..]);"] # [doc = " ```"] # [doc = ""] # [doc = " ---"] # [doc = ""] # [doc = " [`KmacXof`] can be created only by using [`Kmac::IntoXof`] interface."] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [doc = " [`KmacXof`]: struct.KmacXof.html"] # [doc = " [`Kmac::IntoXof`]: struct.Kmac.html#impl-IntoXof"] # [derive (Clone)] pub struct KmacXof { state : CShake , }
};
}
