// Generated macro for internal (function)
macro_rules! Depcrate_rustc_internalinternal {
() => {
// Module: crate::rustc_internal
// Provides: {"internal"}
// Dependencies: {}
# [doc = " Convert a stable item into its internal Rust compiler counterpart, if one exists."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This function is unstable, and it's behavior may change at any point."] # [doc = " Not every stable item can be converted to an internal one."] # [doc = " Furthermore, items that were previously supported, may no longer be supported in newer versions."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if rustc_public has not been properly initialized."] pub fn internal < 'tcx , S > (tcx : TyCtxt < 'tcx > , item : S) -> S :: T < 'tcx > where S : RustcInternal , { with_container (| tables , _ | item . internal (tables , tcx)) }
};
}
