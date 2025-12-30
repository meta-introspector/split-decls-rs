// Generated macro for impl_142 (impl)
macro_rules! Depcrate_request_builderimpl_142 {
() => {
// Module: crate::request::builder
// Provides: {"impl_142"}
// Dependencies: {}
impl < P > CertificateBuilder < P > where P : BuilderProfile , { # [doc = " Creates a new certificate builder"] pub fn new (profile : P , serial_number : SerialNumber , mut validity : Validity , subject_public_key_info : SubjectPublicKeyInfo ,) -> Result < Self > { let signature_alg = AlgorithmIdentifier { oid : NULL_OID , parameters : None , } ; let subject = profile . get_subject () ; let issuer = profile . get_issuer (& subject) ; validity . not_before . rfc5280_adjust_utc_time () ? ; validity . not_after . rfc5280_adjust_utc_time () ? ; let tbs = TbsCertificate { version : Version :: V3 , serial_number , signature : signature_alg , issuer , validity , subject , subject_public_key_info , extensions : None , issuer_unique_id : None , subject_unique_id : None , } ; let extensions = Extensions :: default () ; Ok (Self { tbs , extensions , profile , }) } # [doc = " Add an extension to this certificate"] # [doc = ""] # [doc = " Extensions need to implement [`AsExtension`], examples may be found in"] # [doc = " in [`AsExtension` documentation](../ext/trait.AsExtension.html#examples) or"] # [doc = " [the implementors](../ext/trait.AsExtension.html#implementors)."] pub fn add_extension < E : AsExtension > (& mut self , extension : & E) -> Result < () > { let ext = extension . to_extension (& self . tbs . subject , & self . extensions) ? ; self . extensions . push (ext) ; Ok (()) } }
};
}
