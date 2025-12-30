// Generated macro for Secp256k1RecoverError (enum)
macro_rules! DepcrateSecp256k1RecoverError {
() => {
// Module: crate
// Provides: {"Secp256k1RecoverError"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Error)] pub enum Secp256k1RecoverError { # [error ("The hash provided to a secp256k1_recover is invalid")] InvalidHash , # [error ("The recovery_id provided to a secp256k1_recover is invalid")] InvalidRecoveryId , # [error ("The signature provided to a secp256k1_recover is invalid")] InvalidSignature , }
};
}
