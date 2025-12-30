// Generated macro for impl_90 (impl)
macro_rules! Depcrate_nameimpl_90 {
() => {
// Module: crate::name
// Provides: {"impl_90"}
// Dependencies: {}
impl Name { # [doc = " Build a name from an [`RdnSequence`]."] # [doc = ""] # [doc = ""] # [doc = " This is provided as an escape hatch (see [RFC 5280 Section 4.1.2.4]) to build"] # [doc = " names from `bmpString`, `TeletexString`, or `UniversalString`:"] # [doc = " ```text"] # [doc = " When CAs have previously issued certificates with issuer fields with"] # [doc = " attributes encoded using TeletexString, BMPString, or"] # [doc = " UniversalString, then the CA MAY continue to use these encodings of"] # [doc = " the DirectoryString to preserve backward compatibility."] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As the name implies, this is a dangerous helper. You are responsible for ensuring the"] # [doc = " [`RdnSequence`] complies with the RFC."] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.2.4]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.4"] # [cfg (feature = "hazmat")] pub fn hazmat_from_rdn_sequence (value : RdnSequence) -> Self { Self (value) } }
};
}
