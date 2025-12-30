// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl crypto :: Signer for SigningKey { fn sign (self : Box < Self > , _message : & [u8]) -> Result < Vec < u8 > , Error > { Ok (SIGNATURE . to_vec ()) } fn scheme (& self) -> SignatureScheme { SIGNATURE_SCHEME } }
};
}
