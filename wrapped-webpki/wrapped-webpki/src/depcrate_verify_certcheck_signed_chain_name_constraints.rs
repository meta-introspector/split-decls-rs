// Generated macro for check_signed_chain_name_constraints (function)
macro_rules! Depcrate_verify_certcheck_signed_chain_name_constraints {
() => {
// Module: crate::verify_cert
// Provides: {"check_signed_chain_name_constraints"}
// Dependencies: {}
fn check_signed_chain_name_constraints (path : & PathNode < '_ > , trust_anchor : & TrustAnchor < '_ > , budget : & mut Budget ,) -> Result < () , ControlFlow < Error , Error > > { let mut name_constraints = trust_anchor . name_constraints . as_ref () . map (| der | untrusted :: Input :: from (der . as_ref ())) ; for path in path . iter () { untrusted :: read_all_optional (name_constraints , Error :: BadDer , | value | { subject_name :: check_name_constraints (value , & path , budget) }) ? ; name_constraints = path . cert . name_constraints ; } Ok (()) }
};
}
