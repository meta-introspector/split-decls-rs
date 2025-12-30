// Generated macro for impl_180 (impl)
macro_rules! Depcrate_verifying_keyimpl_180 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_180"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P : ParameterSet > EncodePublicKey for VerifyingKey < P > { fn to_public_key_der (& self) -> pkcs8 :: spki :: Result < der :: Document > { let algorithm_identifier = pkcs8 :: AlgorithmIdentifierRef { oid : P :: ALGORITHM_OID , parameters : None , } ; let public_key = self . to_bytes () ; let subject_public_key = der :: asn1 :: BitStringRef :: new (0 , & public_key) ? ; pkcs8 :: SubjectPublicKeyInfo { algorithm : algorithm_identifier , subject_public_key , } . try_into () } }
};
}
