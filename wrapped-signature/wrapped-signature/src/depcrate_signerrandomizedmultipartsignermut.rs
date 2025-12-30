// Generated macro for RandomizedMultipartSignerMut (trait)
macro_rules! Depcrate_signerRandomizedMultipartSignerMut {
() => {
// Module: crate::signer
// Provides: {"RandomizedMultipartSignerMut"}
// Dependencies: {}
# [doc = " Equivalent of [`RandomizedSignerMut`] but the message is provided in non-contiguous byte slices."] # [cfg (feature = "rand_core")] pub trait RandomizedMultipartSignerMut < S > { # [doc = " Equivalent of [`RandomizedSignerMut::sign_with_rng()`] but"] # [doc = " the message is provided in non-contiguous byte slices."] fn multipart_sign_with_rng < R : CryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [& [u8]]) -> S { self . try_multipart_sign_with_rng (rng , msg) . expect ("signature operation failed") } # [doc = " Equivalent of [`RandomizedSignerMut::try_sign_with_rng()`]"] # [doc = " but the message is provided in non-contiguous byte slices."] fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [& [u8]] ,) -> Result < S , Error > ; }
};
}
