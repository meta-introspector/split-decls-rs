// Generated macro for SecCertificateExt (trait)
macro_rules! Depcrate_os_macos_certificateSecCertificateExt {
() => {
// Module: crate::os::macos::certificate
// Provides: {"SecCertificateExt"}
// Dependencies: {}
# [doc = " An extension trait adding OSX specific functionality to `SecCertificate`."] pub trait SecCertificateExt { # [doc = " Returns the common name associated with the certificate."] fn common_name (& self) -> Result < String , Error > ; # [doc = " Returns the public key associated with the certificate."] # [cfg_attr (not (feature = "OSX_10_14") , deprecated (note = "Uses deprecated SecCertificateCopyPublicKey. Enable OSX_10_14 feature to avoid it"))] fn public_key (& self) -> Result < SecKey , Error > ; # [doc = " Returns the set of properties associated with the certificate."] # [doc = ""] # [doc = " The `keys` argument can optionally be used to filter the properties loaded to an explicit"] # [doc = " subset."] fn properties (& self , keys : Option < & [CertificateOid] >) -> Result < CertificateProperties , CFError > ; # [doc = " Returns the SHA-256 fingerprint of the certificate."] fn fingerprint (& self) -> Result < [u8 ; 32] , CFError > { unimplemented ! () } }
};
}
