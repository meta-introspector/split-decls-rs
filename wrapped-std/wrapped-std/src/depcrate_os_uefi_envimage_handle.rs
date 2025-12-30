// Generated macro for IMAGE_HANDLE (static)
macro_rules! Depcrate_os_uefi_envIMAGE_HANDLE {
() => {
// Module: crate::os::uefi::env
// Provides: {"IMAGE_HANDLE"}
// Dependencies: {}
static IMAGE_HANDLE : Atomic < * mut c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ;
};
}
