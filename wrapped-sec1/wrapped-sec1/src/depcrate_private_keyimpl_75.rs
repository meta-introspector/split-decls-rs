// Generated macro for impl_75 (impl)
macro_rules! Depcrate_private_keyimpl_75 {
() => {
// Module: crate::private_key
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < & EcPrivateKey < '_ > > for SecretDocument { type Error = Error ; fn try_from (private_key : & EcPrivateKey < '_ >) -> Result < Self > { Ok (Self :: encode_msg (private_key) ?) } }
};
}
