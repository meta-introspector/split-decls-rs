// Generated macro for with_container (function)
macro_rules! Depcrate_rustc_internalwith_container {
() => {
// Module: crate::rustc_internal
// Provides: {"with_container"}
// Dependencies: {}
# [doc = " Loads the current context and calls a function with it."] # [doc = " Do not nest these, as that will ICE."] pub (crate) fn with_container < R , B : Bridge > (f : impl for < 'tcx > FnOnce (& mut Tables < 'tcx , B > , & CompilerCtxt < 'tcx , B >) -> R ,) -> R { assert ! (TLV . is_set ()) ; TLV . with (| tlv | { let ptr = tlv . get () ; assert ! (! ptr . is_null ()) ; let container = ptr as * const Container < '_ , B > ; let mut tables = unsafe { (* container) . tables . borrow_mut () } ; let cx = unsafe { (* container) . cx . borrow () } ; f (& mut * tables , & * cx) }) }
};
}
