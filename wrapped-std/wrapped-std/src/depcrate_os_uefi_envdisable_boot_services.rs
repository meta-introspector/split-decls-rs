// Generated macro for disable_boot_services (function)
macro_rules! Depcrate_os_uefi_envdisable_boot_services {
() => {
// Module: crate::os::uefi::env
// Provides: {"disable_boot_services"}
// Dependencies: {}
pub (crate) fn disable_boot_services () { BOOT_SERVICES_FLAG . store (false , Ordering :: Release) }
};
}
