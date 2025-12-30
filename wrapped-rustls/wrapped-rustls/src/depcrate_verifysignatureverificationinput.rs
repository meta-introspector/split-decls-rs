// Generated macro for SignatureVerificationInput (struct)
macro_rules! Depcrate_verifySignatureVerificationInput {
() => {
// Module: crate::verify
// Provides: {"SignatureVerificationInput"}
// Dependencies: {}
# [doc = " Input for message signature verification."] # [non_exhaustive] # [derive (Debug)] pub struct SignatureVerificationInput < 'a > { # [doc = " The message is not hashed, and needs hashing during verification."] pub message : & 'a [u8] , # [doc = " The public key to use."] # [doc = ""] # [doc = " `signer` has already been validated by the point this is called."] pub signer : & 'a SignerPublicKey < 'a > , # [doc = " The signature scheme and payload."] pub signature : & 'a DigitallySignedStruct , }
};
}
