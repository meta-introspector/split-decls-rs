// Generated macro for EkuValidationMode (enum)
macro_rules! Depcrate_verify_certEkuValidationMode {
() => {
// Module: crate::verify_cert
// Provides: {"EkuValidationMode"}
// Dependencies: {}
# [doc = " Extended Key Usage (EKU) of a certificate."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum EkuValidationMode { # [doc = " The certificate must contain the specified [`KeyPurposeId`] as EKU."] Required (KeyPurposeId < 'static >) , # [doc = " If the certificate has EKUs, then the specified [`KeyPurposeId`] must be included."] RequiredIfPresent (KeyPurposeId < 'static >) , }
};
}
