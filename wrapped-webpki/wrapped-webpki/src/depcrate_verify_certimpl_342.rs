// Generated macro for impl_342 (impl)
macro_rules! Depcrate_verify_certimpl_342 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'a , 'r > Iterator for KeyPurposeIdIter < 'a , 'r > { type Item = Result < KeyPurposeId < 'a > , Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . input . at_end () { return None ; } Some (der :: expect_tag (self . input , der :: Tag :: OID) . map (| oid_value | KeyPurposeId { oid_value })) } }
};
}
