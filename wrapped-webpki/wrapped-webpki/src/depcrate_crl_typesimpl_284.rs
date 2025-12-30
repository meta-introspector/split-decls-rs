// Generated macro for impl_284 (impl)
macro_rules! Depcrate_crl_typesimpl_284 {
() => {
// Module: crate::crl::types
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for RevocationReason { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { let input = der :: expect_tag (reader , Tag :: Enum) ? ; Self :: try_from (input . read_all (Error :: BadDer , | reason | { reason . read_byte () . map_err (| _ | Error :: BadDer) }) ?) } const TYPE_ID : DerTypeId = DerTypeId :: RevocationReason ; }
};
}
