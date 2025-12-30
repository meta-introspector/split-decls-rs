// Generated macro for version3 (function)
macro_rules! Depcrate_certversion3 {
() => {
// Module: crate::cert
// Provides: {"version3"}
// Dependencies: {}
fn version3 (input : & mut untrusted :: Reader < '_ >) -> Result < () , Error > { der :: nested (input , der :: Tag :: ContextSpecificConstructed0 , Error :: UnsupportedCertVersion , | input | { let version = u8 :: from_der (input) ? ; if version != 2 { return Err (Error :: UnsupportedCertVersion) ; } Ok (()) } ,) }
};
}
