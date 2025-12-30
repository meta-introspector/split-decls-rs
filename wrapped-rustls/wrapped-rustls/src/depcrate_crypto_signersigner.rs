// Generated macro for Signer (trait)
macro_rules! Depcrate_crypto_signerSigner {
() => {
// Module: crate::crypto::signer
// Provides: {"Signer"}
// Dependencies: {}
# [doc = " A thing that can sign a message."] pub trait Signer : Debug + Send + Sync { # [doc = " Signs `message` using the selected scheme."] # [doc = ""] # [doc = " `message` is not hashed; the implementer must hash it using the hash function"] # [doc = " implicit in [`Self::scheme()`]."] # [doc = ""] # [doc = " The returned signature format is also defined by [`Self::scheme()`]."] fn sign (self : Box < Self > , message : & [u8]) -> Result < Vec < u8 > , Error > ; # [doc = " Reveals which scheme will be used when you call [`Self::sign()`]."] fn scheme (& self) -> SignatureScheme ; }
};
}
