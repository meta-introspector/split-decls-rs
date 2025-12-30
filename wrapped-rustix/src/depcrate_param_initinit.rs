// Generated macro for init (function)
macro_rules! Depcrate_param_initinit {
() => {
// Module: crate::param::init
// Provides: {"init"}
// Dependencies: {}
# [doc = " Initialize process-wide state."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This must be passed a pointer to the original environment variable block"] # [doc = " set up by the OS at process startup, and it must be called before any other"] # [doc = " rustix functions are called."] # [inline] # [doc (hidden)] pub unsafe fn init (envp : * mut * mut u8) { backend :: param :: auxv :: init (envp) }
};
}
