// Generated macro for SetKeyProvInfo (struct)
macro_rules! Depcrate_cert_contextSetKeyProvInfo {
() => {
// Module: crate::cert_context
// Provides: {"SetKeyProvInfo"}
// Dependencies: {}
# [doc = " A builder used to set the private key associated with a certificate."] pub struct SetKeyProvInfo < 'a > { cert : & 'a CertContext , container : Option < Vec < u16 > > , provider : Option < Vec < u16 > > , type_ : u32 , flags : u32 , key_spec : u32 , }
};
}
