// Generated macro for impl_370 (impl)
macro_rules! Depcrate_x509impl_370 {
() => {
// Module: crate::x509
// Provides: {"impl_370"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for Extension < 'a > { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { let id = der :: expect_tag (reader , der :: Tag :: OID) ? ; let critical = bool :: from_der (reader) ? ; let value = der :: expect_tag (reader , der :: Tag :: OctetString) ? ; Ok (Extension { id , critical , value , }) } const TYPE_ID : DerTypeId = DerTypeId :: Extension ; }
};
}
