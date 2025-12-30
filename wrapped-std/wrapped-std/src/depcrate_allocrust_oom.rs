// Generated macro for rust_oom (function)
macro_rules! Depcrate_allocrust_oom {
() => {
// Module: crate::alloc
// Provides: {"rust_oom"}
// Dependencies: {}
# [cfg (not (test))] # [doc (hidden)] # [alloc_error_handler] # [unstable (feature = "alloc_internals" , issue = "none")] pub fn rust_oom (layout : Layout) -> ! { let hook = HOOK . load (Ordering :: Acquire) ; let hook : fn (Layout) = if hook . is_null () { default_alloc_error_hook } else { unsafe { mem :: transmute (hook) } } ; hook (layout) ; crate :: process :: abort () }
};
}
