// Generated macro for NT_INFORMATION (function)
macro_rules! Depcrate_shared_ntdefNT_INFORMATION {
() => {
// Module: crate::shared::ntdef
// Provides: {"NT_INFORMATION"}
// Dependencies: {}
# [inline] pub fn NT_INFORMATION (Status : NTSTATUS) -> bool { ((Status as ULONG) >> 30) == 1 }
};
}
