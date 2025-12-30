// Generated macro for macro_42772 (macro)
macro_rules! Depcrate_um_wincryptmacro_42772 {
() => {
// Module: crate::um::wincrypt
// Provides: {"macro_42772"}
// Dependencies: {}
STRUCT ! { struct CMSG_CONTENT_ENCRYPT_INFO { cbSize : DWORD , hCryptProv : HCRYPTPROV_LEGACY , ContentEncryptionAlgorithm : CRYPT_ALGORITHM_IDENTIFIER , pvEncryptionAuxInfo : * mut c_void , cRecipients : DWORD , rgCmsRecipients : PCMSG_RECIPIENT_ENCODE_INFO , pfnAlloc : PFN_CMSG_ALLOC , pfnFree : PFN_CMSG_FREE , dwEncryptFlags : DWORD , u : CMSG_CONTENT_ENCRYPT_INFO_u , dwFlags : DWORD , fCNG : BOOL , pbCNGContentEncryptKeyObject : * mut BYTE , pbContentEncryptKey : * mut BYTE , cbContentEncryptKey : DWORD , } }
};
}
