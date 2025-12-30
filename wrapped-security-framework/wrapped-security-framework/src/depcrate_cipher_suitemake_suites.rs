// Generated macro for make_suites (macro)
macro_rules! Depcrate_cipher_suitemake_suites {
() => {
// Module: crate::cipher_suite
// Provides: {"make_suites"}
// Dependencies: {}
macro_rules ! make_suites { ($ ($ suite : ident) ,+) => { # [doc = " TLS cipher suites."] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct CipherSuite (SSLCipherSuite) ; # [allow (missing_docs)] impl CipherSuite { $ (pub const $ suite : Self = Self ($ suite) ;) + # [inline (always)] # [must_use] pub const fn from_raw (raw : SSLCipherSuite) -> Self { Self (raw) } # [inline (always)] # [must_use] pub const fn to_raw (& self) -> SSLCipherSuite { self . 0 } } } }
};
}
