// Generated macro for impl_35 (impl)
macro_rules! Depcrate_agentimpl_35 {
() => {
// Module: crate::agent
// Provides: {"impl_35"}
// Dependencies: {}
impl PublicKey { unsafe fn from_raw (raw : * mut raw :: libssh2_agent_publickey) -> Self { let blob = slice :: from_raw_parts_mut ((* raw) . blob , (* raw) . blob_len as usize) ; let comment = (* raw) . comment ; let comment = if comment . is_null () { String :: new () } else { CStr :: from_ptr (comment) . to_string_lossy () . into_owned () } ; Self { blob : blob . to_vec () , comment , } } # [doc = " Return the data of this public key."] pub fn blob (& self) -> & [u8] { & self . blob } # [doc = " Returns the comment in a printable format"] pub fn comment (& self) -> & str { & self . comment } }
};
}
