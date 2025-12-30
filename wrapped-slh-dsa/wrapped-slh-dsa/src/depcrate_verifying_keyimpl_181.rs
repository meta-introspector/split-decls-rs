// Generated macro for impl_181 (impl)
macro_rules! Depcrate_verifying_keyimpl_181 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_181"}
// Dependencies: {}
impl < P : ParameterSet > TryFrom < pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for VerifyingKey < P > { type Error = spki :: Error ; fn try_from (spki : pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> spki :: Result < Self > { spki . algorithm . assert_algorithm_oid (P :: ALGORITHM_OID) ? ; Ok (Self :: try_from (spki . subject_public_key . as_bytes () . ok_or_else (| | der :: Tag :: BitString . value_error () . to_error ()) ? ,) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) ?) } }
};
}
