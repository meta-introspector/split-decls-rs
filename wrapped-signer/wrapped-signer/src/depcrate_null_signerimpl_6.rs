// Generated macro for impl_6 (impl)
macro_rules! Depcrate_null_signerimpl_6 {
() => {
// Module: crate::null_signer
// Provides: {"impl_6"}
// Dependencies: {}
impl Signer for NullSigner { fn try_pubkey (& self) -> Result < Pubkey , SignerError > { Ok (self . pubkey) } fn try_sign_message (& self , _message : & [u8]) -> Result < Signature , SignerError > { Ok (Signature :: default ()) } fn is_interactive (& self) -> bool { false } }
};
}
