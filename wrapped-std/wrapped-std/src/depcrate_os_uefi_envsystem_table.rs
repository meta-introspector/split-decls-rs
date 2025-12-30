// Generated macro for SYSTEM_TABLE (static)
macro_rules! Depcrate_os_uefi_envSYSTEM_TABLE {
() => {
// Module: crate::os::uefi::env
// Provides: {"SYSTEM_TABLE"}
// Dependencies: {}
static SYSTEM_TABLE : Atomic < * mut c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ;
};
}
