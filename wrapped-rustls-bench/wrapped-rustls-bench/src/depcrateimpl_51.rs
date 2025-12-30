// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl From < RequestedKeyType > for KeyType { fn from (val : RequestedKeyType) -> Self { match val { RequestedKeyType :: Rsa2048 => Self :: Rsa2048 , RequestedKeyType :: EcdsaP256 => Self :: EcdsaP256 , RequestedKeyType :: EcdsaP384 => Self :: EcdsaP384 , RequestedKeyType :: Ed25519 => Self :: Ed25519 , } } }
};
}
