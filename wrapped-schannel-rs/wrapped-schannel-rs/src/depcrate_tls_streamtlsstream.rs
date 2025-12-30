// Generated macro for TlsStream (struct)
macro_rules! Depcrate_tls_streamTlsStream {
() => {
// Module: crate::tls_stream
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " An Schannel TLS stream."] pub struct TlsStream < S > { cred : SchannelCred , context : SecurityContext , cert_store : Option < CertStore > , domain : Option < Vec < u16 > > , use_sni : bool , accept_invalid_hostnames : bool , verify_callback : Option < Arc < dyn Fn (CertValidationResult) -> io :: Result < () > + Sync + Send > > , stream : S , state : State , server : bool , accept_first : bool , needs_read : usize , dec_in : Cursor < Vec < u8 > > , enc_in : Cursor < Vec < u8 > > , out_buf : Cursor < Vec < u8 > > , # [doc = " the (unencrypted) length of the last write call used to track writes"] last_write_len : usize , requested_application_protocols : Option < Vec < Vec < u8 > > > , }
};
}
