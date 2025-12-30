// Generated macro for take_alloc_error_hook (function)
macro_rules! Depcrate_alloctake_alloc_error_hook {
() => {
// Module: crate::alloc
// Provides: {"take_alloc_error_hook"}
// Dependencies: {}
# [doc = " Unregisters the current allocation error hook, returning it."] # [doc = ""] # [doc = " *See also the function [`set_alloc_error_hook`].*"] # [doc = ""] # [doc = " If no custom hook is registered, the default hook will be returned."] # [unstable (feature = "alloc_error_hook" , issue = "51245")] pub fn take_alloc_error_hook () -> fn (Layout) { let hook = HOOK . swap (ptr :: null_mut () , Ordering :: Acquire) ; if hook . is_null () { default_alloc_error_hook } else { unsafe { mem :: transmute (hook) } } }
};
}
