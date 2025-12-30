// Generated macro for KernelSigrestore (type)
macro_rules! Depcrate_runtimeKernelSigrestore {
() => {
// Module: crate::runtime
// Provides: {"KernelSigrestore"}
// Dependencies: {}
# [doc = " `__sigrestore_t`"] # [doc = ""] # [doc = " This type differs from `libc::sigrestore_t`, but can be transmuted to it."] pub type KernelSigrestore = Option < unsafe extern "C" fn () > ;
};
}
