// Generated macro for InconsistentKeys (enum)
macro_rules! Depcrate_errorInconsistentKeys {
() => {
// Module: crate::error
// Provides: {"InconsistentKeys"}
// Dependencies: {}
# [doc = " Specific failure cases from [`Credentials::new()`] or a [`crate::crypto::SigningKey`] that cannot produce a corresponding public key."] # [doc = ""] # [doc = " [`Credentials::new()`]: crate::crypto::Credentials::new()"] # [non_exhaustive] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum InconsistentKeys { # [doc = " The public key returned by the [`SigningKey`] does not match the public key information in the certificate."] # [doc = ""] # [doc = " [`SigningKey`]: crate::crypto::SigningKey"] KeyMismatch , # [doc = " The [`SigningKey`] cannot produce its corresponding public key."] # [doc = ""] # [doc = " [`SigningKey`]: crate::crypto::SigningKey"] Unknown , }
};
}
