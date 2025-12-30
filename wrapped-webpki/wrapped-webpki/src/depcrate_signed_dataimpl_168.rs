// Generated macro for impl_168 (impl)
macro_rules! Depcrate_signed_dataimpl_168 {
() => {
// Module: crate::signed_data
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for SubjectPublicKeyInfo < 'a > { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { let algorithm_id_value = der :: expect_tag (reader , der :: Tag :: Sequence) ? ; let key_value = der :: bit_string_with_no_unused_bits (reader) ? ; Ok (SubjectPublicKeyInfo { algorithm_id_value , key_value , }) } const TYPE_ID : DerTypeId = DerTypeId :: SubjectPublicKeyInfo ; }
};
}
