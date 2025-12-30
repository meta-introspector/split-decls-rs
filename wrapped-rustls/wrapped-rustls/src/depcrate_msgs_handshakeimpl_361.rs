// Generated macro for impl_361 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_361 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_361"}
// Dependencies: {}
impl Codec < '_ > for CertificateRequestExtensions { fn encode (& self , bytes : & mut Vec < u8 >) { let extensions = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; for ext in Self :: ALL_EXTENSIONS { self . encode_one (* ext , extensions . buf) ; } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let mut out = Self :: default () ; let mut checker = DuplicateExtensionChecker :: new () ; let len = usize :: from (u16 :: read (r) ?) ; let mut sub = r . sub (len) ? ; while sub . any_left () { out . read_one (& mut sub , | unknown | checker . check (unknown)) ? ; } if out . signature_algorithms . as_ref () . map (| algs | algs . is_empty ()) . unwrap_or_default () { return Err (InvalidMessage :: NoSignatureSchemes) ; } Ok (out) } }
};
}
