// Generated macro for try_system_table (function)
macro_rules! Depcrate_os_uefi_envtry_system_table {
() => {
// Module: crate::os::uefi::env
// Provides: {"try_system_table"}
// Dependencies: {}
# [doc = " Gets the SystemTable Pointer."] # [doc = ""] # [doc = " This function is mostly intended for places where panic is not an option."] pub (crate) fn try_system_table () -> Option < NonNull < c_void > > { NonNull :: new (SYSTEM_TABLE . load (Ordering :: Acquire)) }
};
}
