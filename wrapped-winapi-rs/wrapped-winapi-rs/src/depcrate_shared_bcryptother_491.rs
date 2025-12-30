// Generated macro for other_491 (other)
macro_rules! Depcrate_shared_bcryptother_491 {
() => {
// Module: crate::shared::bcrypt
// Provides: {"other_491"}
// Dependencies: {}
extern "system" { pub fn BCryptEnumProviders (pszAlgId : LPCWSTR , pImplCount : * mut ULONG , ppImplList : * mut * mut BCRYPT_PROVIDER_NAME , dwFlags : ULONG ,) -> NTSTATUS ; }
};
}
