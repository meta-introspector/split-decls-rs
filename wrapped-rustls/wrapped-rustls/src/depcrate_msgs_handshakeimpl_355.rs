// Generated macro for impl_355 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_355 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_355"}
// Dependencies: {}
impl DistinguishedName { # [doc = " Create a [`DistinguishedName`] after prepending its outer SEQUENCE encoding."] # [doc = ""] # [doc = " This can be decoded using [x509-parser's FromDer trait](https://docs.rs/x509-parser/latest/x509_parser/prelude/trait.FromDer.html)."] # [doc = ""] # [doc = " ```ignore"] # [doc = " use x509_parser::prelude::FromDer;"] # [doc = " println!(\"{}\", x509_parser::x509::X509Name::from_der(dn.as_ref())?.1);"] # [doc = " ```"] pub fn in_sequence (bytes : & [u8]) -> Self { Self (PayloadU16 :: new (wrap_in_sequence (bytes))) } }
};
}
