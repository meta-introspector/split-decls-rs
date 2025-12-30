// Generated macro for PrehashVerifier (trait)
macro_rules! Depcrate_hazmatPrehashVerifier {
() => {
// Module: crate::hazmat
// Provides: {"PrehashVerifier"}
// Dependencies: {}
# [doc = " Verify the provided message prehash using `Self` (e.g. a public key)"] pub trait PrehashVerifier < S > { # [doc = " Use `Self` to verify that the provided signature for a given message"] # [doc = " `prehash` is authentic."] # [doc = ""] # [doc = " The `prehash` parameter should be the output of a secure cryptographic"] # [doc = " hash function."] # [doc = ""] # [doc = " Returns `Error` if it is inauthentic or some other error occurred, or"] # [doc = " otherwise returns `Ok(())`."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Security Warning"] # [doc = ""] # [doc = " If `prehash` is something other than the output of a cryptographically"] # [doc = " secure hash function, an attacker can potentially forge signatures by"] # [doc = " solving a system of linear equations."] fn verify_prehash (& self , prehash : & [u8] , signature : & S) -> Result < () , Error > ; }
};
}
