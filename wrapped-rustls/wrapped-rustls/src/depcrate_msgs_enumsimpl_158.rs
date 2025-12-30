// Generated macro for impl_158 (impl)
macro_rules! Depcrate_msgs_enumsimpl_158 {
() => {
// Module: crate::msgs::enums
// Provides: {"impl_158"}
// Dependencies: {}
impl ExtensionType { # [doc = " Returns true if the extension type can be compressed in an \"inner\" client hello for ECH."] # [doc = ""] # [doc = " This function should only return true for extension types where the inner hello and outer"] # [doc = " hello extensions values will always be identical. Extensions that may be identical"] # [doc = " sometimes (e.g. server name, cert compression methods), but not always, SHOULD NOT be"] # [doc = " compressed."] # [doc = ""] # [doc = " See [draft-ietf-tls-esni-18 §5](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-5)"] # [doc = " and [draft-ietf-tls-esni-18 §10.5](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-10.5)"] # [doc = " for more information."] pub (crate) fn ech_compress (& self) -> bool { matches ! (self , Self :: StatusRequest | Self :: EllipticCurves | Self :: SignatureAlgorithms | Self :: SignatureAlgorithmsCert | Self :: ALProtocolNegotiation | Self :: SupportedVersions | Self :: Cookie | Self :: KeyShare | Self :: PSKKeyExchangeModes) } }
};
}
