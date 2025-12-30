// Generated macro for AsyncDigestSigner (trait)
macro_rules! Depcrate_signerAsyncDigestSigner {
() => {
// Module: crate::signer
// Provides: {"AsyncDigestSigner"}
// Dependencies: {}
# [doc = " Asynchronously sign the given prehashed message `Digest` using `Self`."] # [doc = ""] # [doc = " This trait is an async equivalent of the [`DigestSigner`] trait."] # [cfg (feature = "digest")] pub trait AsyncDigestSigner < D , S > where D : Update , { # [doc = " Attempt to sign a message by updating the received `Digest` with it,"] # [doc = " returning a digital signature on success, or an error if something went wrong."] # [doc = ""] # [doc = " The given function can be invoked multiple times. It is expected that"] # [doc = " in each invocation the `Digest` is updated with the entire equal message."] async fn sign_digest_async < F : AsyncFn (& mut D) -> Result < () , Error > > (& self , f : F ,) -> Result < S , Error > ; }
};
}
