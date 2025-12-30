// Generated macro for impl_74 (impl)
macro_rules! Depcrate_private_keyimpl_74 {
() => {
// Module: crate::private_key
// Provides: {"impl_74"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < EcPrivateKey < '_ > > for SecretDocument { type Error = Error ; fn try_from (private_key : EcPrivateKey < '_ >) -> Result < Self > { SecretDocument :: try_from (& private_key) } }
};
}
