// Generated macro for other_499 (other)
macro_rules! Depcrate_shared_bcryptother_499 {
() => {
// Module: crate::shared::bcrypt
// Provides: {"other_499"}
// Dependencies: {}
extern "system" { pub fn BCryptGenRandom (hAlgorithm : BCRYPT_ALG_HANDLE , pbBuffer : PUCHAR , cbBuffer : ULONG , dwFlags : ULONG ,) -> NTSTATUS ; pub fn BCryptDeriveKeyCapi (hHash : BCRYPT_HASH_HANDLE , hTargetAlg : BCRYPT_ALG_HANDLE , pbDerivedKey : PUCHAR , cbDerivedKey : ULONG , dwFlags : ULONG ,) -> NTSTATUS ; pub fn BCryptDeriveKeyPBKDF2 (hPrf : BCRYPT_ALG_HANDLE , pbPassword : PUCHAR , cbPassword : ULONG , pbSalt : PUCHAR , cbSalt : ULONG , cIterations : ULONGLONG , pbDerivedKey : PUCHAR , cbDerivedKey : ULONG , dwFlags : ULONG ,) -> NTSTATUS ; }
};
}
