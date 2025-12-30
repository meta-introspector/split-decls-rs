// Generated macro for boot_services (function)
macro_rules! Depcrate_os_uefi_envboot_services {
() => {
// Module: crate::os::uefi::env
// Provides: {"boot_services"}
// Dependencies: {}
# [doc = " Gets the BootServices Pointer."] # [doc = ""] # [doc = " This function also checks if `ExitBootServices` has already been called."] pub fn boot_services () -> Option < NonNull < c_void > > { if BOOT_SERVICES_FLAG . load (Ordering :: Acquire) { let system_table : NonNull < r_efi :: efi :: SystemTable > = try_system_table () ? . cast () ; let boot_services = unsafe { (* system_table . as_ptr ()) . boot_services } ; NonNull :: new (boot_services) . map (| x | x . cast ()) } else { None } }
};
}
