// Generated macro for RandomizedSigner (trait)
macro_rules! Depcrate_signerRandomizedSigner {
() => {
// Module: crate::signer
// Provides: {"RandomizedSigner"}
// Dependencies: {}
# [doc = " Sign the given message using the provided external randomness source."] # [cfg (feature = "rand_core")] pub trait RandomizedSigner < S > { # [doc = " Sign the given message and return a digital signature"] fn sign_with_rng < R : CryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8]) -> S { self . try_sign_with_rng (rng , msg) . expect ("signature operation failed") } # [doc = " Attempt to sign the given message, returning a digital signature on"] # [doc = " success, or an error if something went wrong."] # [doc = ""] # [doc = " The main intended use case for signing errors is when communicating"] # [doc = " with external signers, e.g. cloud KMS, HSMs, or other hardware tokens."] fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > ; }
};
}
