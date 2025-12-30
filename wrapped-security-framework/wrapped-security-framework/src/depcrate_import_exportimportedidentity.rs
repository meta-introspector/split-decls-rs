// Generated macro for ImportedIdentity (struct)
macro_rules! Depcrate_import_exportImportedIdentity {
() => {
// Module: crate::import_export
// Provides: {"ImportedIdentity"}
// Dependencies: {}
# [doc = " Information about an imported identity."] # [derive (Clone)] # [non_exhaustive] pub struct ImportedIdentity { # [doc = " The label of the identity."] pub label : Option < String > , # [doc = " The ID of the identity. Typically the SHA-1 hash of the public key."] pub key_id : Option < Vec < u8 > > , # [doc = " A `SecTrust` object set up to validate this identity."] pub trust : Option < SecTrust > , # [doc = " A certificate chain validating this identity."] pub cert_chain : Option < Vec < SecCertificate > > , # [doc = " The identity itself."] pub identity : Option < SecIdentity > , }
};
}
