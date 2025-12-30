// Generated macro for CertificateMetadata (struct)
macro_rules! DepcrateCertificateMetadata {
() => {
// Module: crate
// Provides: {"CertificateMetadata"}
// Dependencies: {}
# [derive (Debug , Clone , Hash , Eq , PartialEq , Deserialize)] pub struct CertificateMetadata { # [serde (rename = "Common Name or Certificate Name")] pub common_name_or_certificate_name : String , # [serde (rename = "Certificate Serial Number")] pub certificate_serial_number : String , # [serde (rename = "SHA-256 Fingerprint")] pub sha256_fingerprint : String , # [serde (rename = "Trust Bits")] pub trust_bits : String , # [serde (rename = "Distrust for TLS After Date")] pub distrust_for_tls_after_date : String , # [serde (rename = "Mozilla Applied Constraints")] pub mozilla_applied_constraints : String , # [serde (rename = "PEM Info")] pub pem_info : String , }
};
}
