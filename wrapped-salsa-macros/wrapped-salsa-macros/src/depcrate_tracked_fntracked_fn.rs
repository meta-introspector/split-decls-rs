// Generated macro for tracked_fn (function)
macro_rules! Depcrate_tracked_fntracked_fn {
() => {
// Module: crate::tracked_fn
// Provides: {"tracked_fn"}
// Dependencies: {}
pub (crate) fn tracked_fn (args : proc_macro :: TokenStream , item : ItemFn) -> syn :: Result < TokenStream > { let hygiene = Hygiene :: from2 (& item) ; let args : FnArgs = syn :: parse (args) ? ; let db_macro = Macro { hygiene , args } ; db_macro . try_fn (item) }
};
}
