// Generated macro for check_validity (function)
macro_rules! Depcrate_verify_certcheck_validity {
() => {
// Module: crate::verify_cert
// Provides: {"check_validity"}
// Dependencies: {}
fn check_validity (input : & mut untrusted :: Reader < '_ > , time : UnixTime) -> Result < () , Error > { let not_before = UnixTime :: from_der (input) ? ; let not_after = UnixTime :: from_der (input) ? ; if not_before > not_after { return Err (Error :: InvalidCertValidity) ; } if time < not_before { return Err (Error :: CertNotValidYet { time , not_before }) ; } if time > not_after { return Err (Error :: CertExpired { time , not_after }) ; } Ok (()) }
};
}
