// Generated macro for other_27620 (other)
macro_rules! Depcrate_um_dpapiother_27620 {
() => {
// Module: crate::um::dpapi
// Provides: {"other_27620"}
// Dependencies: {}
extern "system" { pub fn CryptProtectData (pDataIn : * mut DATA_BLOB , szDataDescr : LPCWSTR , pOptionalEntropy : * mut DATA_BLOB , pvReserved : PVOID , pPromptStruct : * mut CRYPTPROTECT_PROMPTSTRUCT , dwFlags : DWORD , pDataOut : * mut DATA_BLOB ,) -> BOOL ; pub fn CryptUnprotectData (pDataIn : * mut DATA_BLOB , ppszDataDescr : * mut LPWSTR , pOptionalEntropy : * mut DATA_BLOB , pvReserved : PVOID , pPromptStruct : * mut CRYPTPROTECT_PROMPTSTRUCT , dwFlags : DWORD , pDataOut : * mut DATA_BLOB ,) -> BOOL ; pub fn CryptProtectDataNoUI (pDataIn : * mut DATA_BLOB , szDataDescr : LPCWSTR , pOptionalEntropy : * mut DATA_BLOB , pvReserved : PVOID , pPromptStruct : * mut CRYPTPROTECT_PROMPTSTRUCT , dwFlags : DWORD , pbOptionalPassword : * const BYTE , cbOptionalPassword : DWORD , pDataOut : * mut DATA_BLOB ,) -> BOOL ; pub fn CryptUnprotectDataNoUI (pDataIn : * mut DATA_BLOB , ppszDataDescr : * mut LPWSTR , pOptionalEntropy : * mut DATA_BLOB , pvReserved : PVOID , pPromptStruct : * mut CRYPTPROTECT_PROMPTSTRUCT , dwFlags : DWORD , pbOptionalPassword : * const BYTE , cbOptionalPassword : DWORD , pDataOut : * mut DATA_BLOB ,) -> BOOL ; pub fn CryptUpdateProtectedState (pOldSid : PSID , pwszOldPassword : LPCWSTR , dwFlags : DWORD , pdwSuccessCount : * mut DWORD , pdwFailureCount : * mut DWORD ,) -> BOOL ; }
};
}
