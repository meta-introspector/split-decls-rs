// Generated macro for Signer (trait)
macro_rules! DepcrateSigner {
() => {
// Module: crate
// Provides: {"Signer"}
// Dependencies: {}
# [doc = " The `Signer` trait declares operations that all digital signature providers"] # [doc = " must support. It is the primary interface by which signers are specified in"] # [doc = " `Transaction` signing interfaces"] pub trait Signer { # [doc = " Infallibly gets the implementor's public key. Returns the all-zeros"] # [doc = " `Pubkey` if the implementor has none."] fn pubkey (& self) -> Pubkey { self . try_pubkey () . unwrap_or_default () } # [doc = " Fallibly gets the implementor's public key"] fn try_pubkey (& self) -> Result < Pubkey , SignerError > ; # [doc = " Infallibly produces an Ed25519 signature over the provided `message`"] # [doc = " bytes. Returns the all-zeros `Signature` if signing is not possible."] fn sign_message (& self , message : & [u8]) -> Signature { self . try_sign_message (message) . unwrap_or_default () } # [doc = " Fallibly produces an Ed25519 signature over the provided `message` bytes."] fn try_sign_message (& self , message : & [u8]) -> Result < Signature , SignerError > ; # [doc = " Whether the implementation requires user interaction to sign"] fn is_interactive (& self) -> bool ; }
};
}
