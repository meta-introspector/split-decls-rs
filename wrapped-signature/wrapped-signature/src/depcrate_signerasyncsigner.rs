// Generated macro for AsyncSigner (trait)
macro_rules! Depcrate_signerAsyncSigner {
() => {
// Module: crate::signer
// Provides: {"AsyncSigner"}
// Dependencies: {}
# [doc = " Asynchronously sign the provided message bytestring using `Self`"] # [doc = " (e.g. client for a Cloud KMS or HSM), returning a digital signature."] # [doc = ""] # [doc = " This trait is an async equivalent of the [`Signer`] trait."] pub trait AsyncSigner < S > { # [doc = " Attempt to sign the given message, returning a digital signature on"] # [doc = " success, or an error if something went wrong."] # [doc = ""] # [doc = " The main intended use case for signing errors is when communicating"] # [doc = " with external signers, e.g. cloud KMS, HSMs, or other hardware tokens."] async fn sign_async (& self , msg : & [u8]) -> Result < S , Error > ; }
};
}
