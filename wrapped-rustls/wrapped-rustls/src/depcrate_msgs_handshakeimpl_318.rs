// Generated macro for impl_318 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_318 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_318"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CertificateExtensions < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { let extensions = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; for ext in Self :: ALL_EXTENSIONS { self . encode_one (* ext , extensions . buf) ; } } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let mut out = Self :: default () ; let len = usize :: from (u16 :: read (r) ?) ; let mut sub = r . sub (len) ? ; while sub . any_left () { out . read_one (& mut sub , | _unk | { Err (InvalidMessage :: UnknownCertificateExtension) }) ? ; } Ok (out) } }
};
}
