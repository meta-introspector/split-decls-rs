// Generated macro for CFHandle (struct)
macro_rules! Depcrate_sys_platform_version_darwin_core_foundationCFHandle {
() => {
// Module: crate::sys::platform_version::darwin::core_foundation
// Provides: {"CFHandle"}
// Dependencies: {}
# [doc = " An open handle to the dynamically loaded CoreFoundation framework."] # [doc = ""] # [doc = " This is `dlopen`ed, and later `dlclose`d. This is done to try to avoid"] # [doc = " \"leaking\" the CoreFoundation symbols to the rest of the user's binary if"] # [doc = " they decided to not link CoreFoundation themselves."] # [doc = ""] # [doc = " It is also faster to look up symbols directly via this handle than with"] # [doc = " `RTLD_DEFAULT`."] pub (super) struct CFHandle (* mut c_void) ;
};
}
