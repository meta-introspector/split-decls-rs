// Generated macro for impl_280 (impl)
macro_rules! Depcrate_crl_typesimpl_280 {
() => {
// Module: crate::crl::types
// Provides: {"impl_280"}
// Dependencies: {}
impl < 'a > BorrowedRevokedCert < 'a > { # [doc = " Construct an owned representation of the revoked certificate."] # [cfg (feature = "alloc")] pub fn to_owned (& self) -> OwnedRevokedCert { OwnedRevokedCert { serial_number : self . serial_number . to_vec () , revocation_date : self . revocation_date , reason_code : self . reason_code , invalidity_date : self . invalidity_date , } } fn remember_extension (& mut self , extension : & Extension < 'a >) -> Result < () , Error > { use crate :: x509 :: ExtensionOid :: * ; remember_extension (extension , | id | { match id { Standard (21) => { set_extension_once (& mut self . reason_code , | | der :: read_all (extension . value)) } Standard (24) => set_extension_once (& mut self . invalidity_date , | | { extension . value . read_all (Error :: BadDer , UnixTime :: from_der) }) , Standard (29) => Err (Error :: UnsupportedIndirectCrl) , _ => extension . unsupported () , } }) } }
};
}
