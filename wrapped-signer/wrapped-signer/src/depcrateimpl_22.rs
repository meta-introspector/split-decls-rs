// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " This implements `Signer` for all ptr types - `Box/Rc/Arc/&/&mut` etc"] impl < Container : Deref < Target = impl Signer + ? Sized > > Signer for Container { # [inline] fn pubkey (& self) -> Pubkey { self . deref () . pubkey () } fn try_pubkey (& self) -> Result < Pubkey , SignerError > { self . deref () . try_pubkey () } fn sign_message (& self , message : & [u8]) -> Signature { self . deref () . sign_message (message) } fn try_sign_message (& self , message : & [u8]) -> Result < Signature , SignerError > { self . deref () . try_sign_message (message) } fn is_interactive (& self) -> bool { self . deref () . is_interactive () } }
};
}
