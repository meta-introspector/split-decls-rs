// Generated macro for impl_45 (impl)
macro_rules! Depcrate_signerimpl_45 {
() => {
// Module: crate::signer
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "rand_core")] impl < S , T > AsyncRandomizedSigner < S > for T where T : RandomizedSigner < S > , { async fn try_sign_with_rng_async < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > { self . try_sign_with_rng (rng , msg) } }
};
}
