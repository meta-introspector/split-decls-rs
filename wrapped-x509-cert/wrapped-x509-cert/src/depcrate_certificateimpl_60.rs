// Generated macro for impl_60 (impl)
macro_rules! Depcrate_certificateimpl_60 {
() => {
// Module: crate::certificate
// Provides: {"impl_60"}
// Dependencies: {}
impl < P : Profile > CertificateInner < P > { # [doc = " Get the [`TbsCertificateInner`] (i.e. the part the signature is computed over)."] pub fn tbs_certificate (& self) -> & TbsCertificateInner < P > { & self . tbs_certificate } # [doc = " Signature algorithm used to sign the serialization of [`CertificateInner::tbs_certificate`]."] pub fn signature_algorithm (& self) -> & AlgorithmIdentifier { & self . signature_algorithm } # [doc = " Signature over the DER serialization of [`CertificateInner::tbs_certificate`] using the"] # [doc = " algorithm identified in [`CertificateInner::signature_algorithm`]."] pub fn signature (& self) -> & BitString { & self . signature } }
};
}
