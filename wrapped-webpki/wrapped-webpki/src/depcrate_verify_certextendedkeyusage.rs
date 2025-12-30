// Generated macro for ExtendedKeyUsage (struct)
macro_rules! Depcrate_verify_certExtendedKeyUsage {
() => {
// Module: crate::verify_cert
// Provides: {"ExtendedKeyUsage"}
// Dependencies: {}
# [doc = " The expected key usage of a certificate."] # [doc = ""] # [doc = " This type represents the expected key usage of an end entity certificate. Although for most"] # [doc = " kinds of certificates the extended key usage extension is optional (and so certificates"] # [doc = " not carrying a particular value in the EKU extension are acceptable). If the extension"] # [doc = " is present, the certificate MUST only be used for one of the purposes indicated."] # [doc = ""] # [doc = " <https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12>"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct ExtendedKeyUsage { inner : EkuValidationMode , }
};
}
