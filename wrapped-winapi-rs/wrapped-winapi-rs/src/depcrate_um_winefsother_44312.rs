// Generated macro for other_44312 (other)
macro_rules! Depcrate_um_winefsother_44312 {
() => {
// Module: crate::um::winefs
// Provides: {"other_44312"}
// Dependencies: {}
extern "system" { pub fn QueryUsersOnEncryptedFile (lpFileName : LPCWSTR , pUsers : * mut PENCRYPTION_CERTIFICATE_HASH_LIST ,) -> DWORD ; pub fn QueryRecoveryAgentsOnEncryptedFile (lpFileName : LPCWSTR , pRecoveryAgents : * mut PENCRYPTION_CERTIFICATE_HASH_LIST ,) -> DWORD ; pub fn RemoveUsersFromEncryptedFile (lpFileName : LPCWSTR , pHashes : PENCRYPTION_CERTIFICATE_HASH_LIST ,) -> DWORD ; pub fn AddUsersToEncryptedFile (lpFileName : LPCWSTR , pEncryptionCertificate : PENCRYPTION_CERTIFICATE_LIST ,) -> DWORD ; pub fn SetUserFileEncryptionKey (pEncryptionCertificate : PENCRYPTION_CERTIFICATE ,) -> DWORD ; pub fn SetUserFileEncryptionKeyEx (pEncryptionCertificate : PENCRYPTION_CERTIFICATE , dwCapabilities : DWORD , dwFlags : DWORD , pvReserved : LPVOID ,) -> DWORD ; pub fn FreeEncryptionCertificateHashList (pUsers : PENCRYPTION_CERTIFICATE_HASH_LIST ,) ; pub fn EncryptionDisable (DirPath : LPCWSTR , Disable : BOOL ,) -> BOOL ; pub fn DuplicateEncryptionInfoFile (SrcFileName : LPCWSTR , DstFileName : LPCWSTR , dwCreationDistribution : DWORD , dwAttributes : DWORD , lpSecurityAttributes : * const SECURITY_ATTRIBUTES ,) -> DWORD ; pub fn GetEncryptedFileMetadata (lpFileName : LPCWSTR , pcbMetadata : PDWORD , ppbMetadata : * mut PBYTE ,) -> DWORD ; pub fn SetEncryptedFileMetadata (lpFileName : LPCWSTR , pbOldMetadata : PBYTE , pbNewMetadata : PBYTE , pOwnerHash : PENCRYPTION_CERTIFICATE_HASH , dwOperation : DWORD , pCertificatesAdded : PENCRYPTION_CERTIFICATE_HASH_LIST ,) -> DWORD ; pub fn FreeEncryptedFileMetadata (pbMetadata : PBYTE ,) ; }
};
}
