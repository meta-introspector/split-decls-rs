// Generated macro for ExtendedKeyUsageValidator (trait)
macro_rules! Depcrate_verify_certExtendedKeyUsageValidator {
() => {
// Module: crate::verify_cert
// Provides: {"ExtendedKeyUsageValidator"}
// Dependencies: {}
# [doc = " A trait for validating the Extended Key Usage (EKU) extensions of a certificate."] pub trait ExtendedKeyUsageValidator { # [doc = " Validate the EKU values in a certificate."] # [doc = ""] # [doc = " `iter` yields the EKU OIDs in the certificate, or an error if the EKU extension"] # [doc = " is malformed. `validate()` should yield `Ok(())` if the EKU values match the"] # [doc = " required policy, or an `Error` if they do not. Ideally the `Error` should be"] # [doc = " `Error::RequiredEkuNotFoundContext` if the policy is not met."] fn validate (& self , iter : KeyPurposeIdIter < '_ , '_ >) -> Result < () , Error > ; }
};
}
