// Generated macro for impl_32 (impl)
macro_rules! Depcrate_msgs_baseimpl_32 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CertificateDer < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { let nest = LengthPrefixedBuffer :: new (Self :: SIZE_LEN , bytes) ; nest . buf . extend (self . as_ref ()) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let len = ListLength :: NonZeroU24 { max : CERTIFICATE_MAX_SIZE_LIMIT , empty_error : InvalidMessage :: IllegalEmptyList ("CertificateDer") , too_many_error : InvalidMessage :: CertificatePayloadTooLarge , } . read (r) ? ; let mut sub = r . sub (len) ? ; let body = sub . rest () ; Ok (Self :: from (body)) } }
};
}
