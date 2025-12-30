// Generated macro for RandomizedSignerMut (trait)
macro_rules! Depcrate_signerRandomizedSignerMut {
() => {
// Module: crate::signer
// Provides: {"RandomizedSignerMut"}
// Dependencies: {}
# [doc = " Sign the provided message bytestring using `&mut Self` (e.g. an evolving"] # [doc = " cryptographic key such as a stateful hash-based signature), and a per-signature"] # [doc = " randomizer, returning a digital signature."] # [cfg (feature = "rand_core")] pub trait RandomizedSignerMut < S > { # [doc = " Sign the given message, update the state, and return a digital signature."] fn sign_with_rng < R : CryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [u8]) -> S { self . try_sign_with_rng (rng , msg) . expect ("signature operation failed") } # [doc = " Attempt to sign the given message, updating the state, and returning a"] # [doc = " digital signature on success, or an error if something went wrong."] # [doc = ""] # [doc = " Signing can fail, e.g., if the number of time periods allowed by the"] # [doc = " current key is exceeded."] fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > ; }
};
}
