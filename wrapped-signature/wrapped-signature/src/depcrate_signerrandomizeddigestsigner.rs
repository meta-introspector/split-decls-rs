// Generated macro for RandomizedDigestSigner (trait)
macro_rules! Depcrate_signerRandomizedDigestSigner {
() => {
// Module: crate::signer
// Provides: {"RandomizedDigestSigner"}
// Dependencies: {}
# [doc = " Combination of [`DigestSigner`] and [`RandomizedSigner`] with support for"] # [doc = " computing a signature over a digest which requires entropy from an RNG."] # [cfg (all (feature = "digest" , feature = "rand_core"))] pub trait RandomizedDigestSigner < D : Update , S > { # [doc = " Sign a message by updating the received `Digest` with it,"] # [doc = " returning a signature."] # [doc = ""] # [doc = " The given function can be invoked multiple times. It is expected that"] # [doc = " in each invocation the `Digest` is updated with the entire equal message."] # [doc = ""] # [doc = " Panics in the event of a signing error."] fn sign_digest_with_rng < R : CryptoRng + ? Sized , F : Fn (& mut D) > (& self , rng : & mut R , f : F) -> S { self . try_sign_digest_with_rng (rng , | digest | { f (digest) ; Ok (()) }) . expect ("signature operation failed") } # [doc = " Attempt to sign a message by updating the received `Digest` with it,"] # [doc = " returning a digital signature on success, or an error if something went wrong."] # [doc = ""] # [doc = " The given function can be invoked multiple times. It is expected that"] # [doc = " in each invocation the `Digest` is updated with the entire equal message."] fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () , Error > > (& self , rng : & mut R , f : F ,) -> Result < S , Error > ; }
};
}
