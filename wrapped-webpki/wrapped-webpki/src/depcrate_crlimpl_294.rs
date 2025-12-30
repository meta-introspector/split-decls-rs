// Generated macro for impl_294 (impl)
macro_rules! Depcrate_crlimpl_294 {
() => {
// Module: crate::crl
// Provides: {"impl_294"}
// Dependencies: {}
impl KeyUsageMode { fn check (self , input : Option < untrusted :: Input < '_ > >) -> Result < () , Error > { let bit_string = match input { Some (input) => { der :: expect_tag (& mut untrusted :: Reader :: new (input) , der :: Tag :: BitString) ? } None => return Ok (()) , } ; let flags = der :: bit_string_flags (bit_string) ? ; # [allow (clippy :: as_conversions)] match flags . bit_set (self as usize) { true => Ok (()) , false => Err (Error :: IssuerNotCrlSigner) , } } }
};
}
