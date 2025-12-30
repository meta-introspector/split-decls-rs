// Generated macro for impl_40 (impl)
macro_rules! Depcrate_signerimpl_40 {
() => {
// Module: crate::signer
// Provides: {"impl_40"}
// Dependencies: {}
# [doc = " Blanket impl of [`RandomizedSignerMut`] for all [`RandomizedSigner`] types."] # [cfg (feature = "rand_core")] impl < S , T : RandomizedSigner < S > > RandomizedSignerMut < S > for T { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > { T :: try_sign_with_rng (self , rng , msg) } }
};
}
