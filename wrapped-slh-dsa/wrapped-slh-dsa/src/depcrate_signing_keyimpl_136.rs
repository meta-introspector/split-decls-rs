// Generated macro for impl_136 (impl)
macro_rules! Depcrate_signing_keyimpl_136 {
() => {
// Module: crate::signing_key
// Provides: {"impl_136"}
// Dependencies: {}
impl < P > TryFrom < pkcs8 :: PrivateKeyInfoRef < '_ > > for SigningKey < P > where P : ParameterSet , { type Error = pkcs8 :: Error ; fn try_from (private_key_info : pkcs8 :: PrivateKeyInfoRef < '_ >) -> pkcs8 :: Result < Self > { private_key_info . algorithm . assert_algorithm_oid (P :: ALGORITHM_OID) ? ; Self :: try_from (private_key_info . private_key . as_bytes ()) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) } }
};
}
