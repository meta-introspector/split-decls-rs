// Generated macro for remember_extension (function)
macro_rules! Depcrate_x509remember_extension {
() => {
// Module: crate::x509
// Provides: {"remember_extension"}
// Dependencies: {}
pub (crate) fn remember_extension (extension : & Extension < '_ > , mut handler : impl FnMut (ExtensionOid) -> Result < () , Error > ,) -> Result < () , Error > { match ExtensionOid :: lookup (extension . id) { Some (oid) => handler (oid) , None => extension . unsupported () , } }
};
}
