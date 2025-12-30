// Generated macro for other_480 (other)
macro_rules! Depcrate_shared_bcryptother_480 {
() => {
// Module: crate::shared::bcrypt
// Provides: {"other_480"}
// Dependencies: {}
extern "system" { pub fn BCryptOpenAlgorithmProvider (phAlgorithm : * mut BCRYPT_ALG_HANDLE , pszAlgId : LPCWSTR , pszImplementation : LPCWSTR , dwFlags : ULONG ,) -> NTSTATUS ; }
};
}
