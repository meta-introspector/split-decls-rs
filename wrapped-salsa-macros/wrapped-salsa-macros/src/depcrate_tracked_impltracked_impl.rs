// Generated macro for tracked_impl (function)
macro_rules! Depcrate_tracked_impltracked_impl {
() => {
// Module: crate::tracked_impl
// Provides: {"tracked_impl"}
// Dependencies: {}
pub (crate) fn tracked_impl (args : proc_macro :: TokenStream , item : syn :: ItemImpl ,) -> syn :: Result < TokenStream > { let hygiene = Hygiene :: from2 (& item) ; let _ : Nothing = syn :: parse (args) ? ; let m = Macro { hygiene } ; let generated = m . try_generate (item) ? ; Ok (generated) }
};
}
