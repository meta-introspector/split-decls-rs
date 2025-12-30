// Generated macro for NT_ERROR (function)
macro_rules! Depcrate_shared_ntdefNT_ERROR {
() => {
// Module: crate::shared::ntdef
// Provides: {"NT_ERROR"}
// Dependencies: {}
# [inline] pub fn NT_ERROR (Status : NTSTATUS) -> bool { ((Status as ULONG) >> 30) == 3 }
};
}
