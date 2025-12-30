// Generated macro for NT_WARNING (function)
macro_rules! Depcrate_shared_ntdefNT_WARNING {
() => {
// Module: crate::shared::ntdef
// Provides: {"NT_WARNING"}
// Dependencies: {}
# [inline] pub fn NT_WARNING (Status : NTSTATUS) -> bool { ((Status as ULONG) >> 30) == 2 }
};
}
