// Generated macro for impl_137 (impl)
macro_rules! Depcrate_signing_keyimpl_137 {
() => {
// Module: crate::signing_key
// Provides: {"impl_137"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P > EncodePrivateKey for SigningKey < P > where P : ParameterSet , { fn to_pkcs8_der (& self) -> pkcs8 :: Result < der :: SecretDocument > { let algorithm_identifier = pkcs8 :: AlgorithmIdentifierRef { oid : P :: ALGORITHM_OID , parameters : None , } ; let private_key = self . to_bytes () ; let pkcs8_key = pkcs8 :: PrivateKeyInfoRef :: new (algorithm_identifier , OctetStringRef :: new (& private_key) ?) ; Ok (der :: SecretDocument :: encode_msg (& pkcs8_key) ?) } }
};
}
