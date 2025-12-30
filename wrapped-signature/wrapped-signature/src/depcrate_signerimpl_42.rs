// Generated macro for impl_42 (impl)
macro_rules! Depcrate_signerimpl_42 {
() => {
// Module: crate::signer
// Provides: {"impl_42"}
// Dependencies: {}
impl < S , T > AsyncSigner < S > for T where T : Signer < S > , { async fn sign_async (& self , msg : & [u8]) -> Result < S , Error > { self . try_sign (msg) } }
};
}
