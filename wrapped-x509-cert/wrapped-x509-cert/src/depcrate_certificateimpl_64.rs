// Generated macro for impl_64 (impl)
macro_rules! Depcrate_certificateimpl_64 {
() => {
// Module: crate::certificate
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "digest")] impl < P > CertificateInner < P > where P : Profile , { # [doc = " Return the hash of the DER serialization of this certificate"] pub fn hash < D > (& self) -> der :: Result < Output < D > > where D : Digest , { let mut digest = D :: new () ; self . encode (& mut DigestWriter (& mut digest)) ? ; Ok (digest . finalize ()) } }
};
}
