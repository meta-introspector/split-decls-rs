// Generated macro for check_issuer_independent_properties (function)
macro_rules! Depcrate_verify_certcheck_issuer_independent_properties {
() => {
// Module: crate::verify_cert
// Provides: {"check_issuer_independent_properties"}
// Dependencies: {}
fn check_issuer_independent_properties (cert : & Cert < '_ > , time : UnixTime , role : Role , sub_ca_count : usize , eku : & dyn ExtendedKeyUsageValidator ,) -> Result < () , Error > { cert . validity . read_all (Error :: BadDer , | value | check_validity (value , time)) ? ; untrusted :: read_all_optional (cert . basic_constraints , Error :: BadDer , | value | { check_basic_constraints (value , role , sub_ca_count) }) ? ; untrusted :: read_all_optional (cert . eku , Error :: BadDer , | input | check_eku (input , eku)) ? ; Ok (()) }
};
}
