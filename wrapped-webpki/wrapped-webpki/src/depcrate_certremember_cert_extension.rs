// Generated macro for remember_cert_extension (function)
macro_rules! Depcrate_certremember_cert_extension {
() => {
// Module: crate::cert
// Provides: {"remember_cert_extension"}
// Dependencies: {}
fn remember_cert_extension < 'a > (cert : & mut Cert < 'a > , extension : & Extension < 'a > ,) -> Result < () , Error > { use ExtensionOid :: * ; remember_extension (extension , | id | { let out = match id { Standard (15) => & mut cert . key_usage , Standard (17) => & mut cert . subject_alt_name , Standard (19) => & mut cert . basic_constraints , Standard (30) => & mut cert . name_constraints , Standard (31) => & mut cert . crl_distribution_points , Standard (37) => & mut cert . eku , _ => return extension . unsupported () , } ; set_extension_once (out , | | { extension . value . read_all (Error :: BadDer , | value | match id { Standard (15) => Ok (value . read_bytes_to_end ()) , _ => der :: expect_tag (value , Tag :: Sequence) , }) }) }) }
};
}
