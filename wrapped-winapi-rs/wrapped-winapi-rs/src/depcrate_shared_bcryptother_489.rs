// Generated macro for other_489 (other)
macro_rules! Depcrate_shared_bcryptother_489 {
() => {
// Module: crate::shared::bcrypt
// Provides: {"other_489"}
// Dependencies: {}
extern "system" { pub fn BCryptEnumAlgorithms (dwAlgOperations : ULONG , pAlgCount : * mut ULONG , ppAlgList : * mut * mut BCRYPT_ALGORITHM_IDENTIFIER , dwFlags : ULONG ,) -> NTSTATUS ; }
};
}
