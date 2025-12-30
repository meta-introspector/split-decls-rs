// Generated macro for Verifier (trait)
macro_rules! Depcrate_verifierVerifier {
() => {
// Module: crate::verifier
// Provides: {"Verifier"}
// Dependencies: {}
# [doc = " Verify the provided message bytestring using `Self` (e.g. a public key)"] pub trait Verifier < S > { # [doc = " Use `Self` to verify that the provided signature for a given message"] # [doc = " bytestring is authentic."] # [doc = ""] # [doc = " Returns `Error` if it is inauthentic, or otherwise returns `()`."] fn verify (& self , msg : & [u8] , signature : & S) -> Result < () , Error > ; }
};
}
