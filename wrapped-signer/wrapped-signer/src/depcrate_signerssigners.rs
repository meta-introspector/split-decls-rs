// Generated macro for Signers (trait)
macro_rules! Depcrate_signersSigners {
() => {
// Module: crate::signers
// Provides: {"Signers"}
// Dependencies: {}
# [doc = " Convenience trait for working with mixed collections of `Signer`s"] pub trait Signers { fn pubkeys (& self) -> Vec < Pubkey > ; fn try_pubkeys (& self) -> Result < Vec < Pubkey > , SignerError > ; fn sign_message (& self , message : & [u8]) -> Vec < Signature > ; fn try_sign_message (& self , message : & [u8]) -> Result < Vec < Signature > , SignerError > ; fn is_interactive (& self) -> bool ; }
};
}
