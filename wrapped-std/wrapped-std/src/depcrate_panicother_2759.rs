// Generated macro for other_2759 (other)
macro_rules! Depcrate_panicother_2759 {
() => {
// Module: crate::panic
// Provides: {"other_2759"}
// Dependencies: {}
# [doc (hidden)] # [unstable (feature = "edition_panic" , issue = "none" , reason = "use panic!() instead")] # [allow_internal_unstable (libstd_sys_internals , const_format_args , panic_internals , rt)] # [cfg_attr (not (test) , rustc_diagnostic_item = "std_panic_2015_macro")] # [rustc_macro_transparency = "semitransparent"] pub macro panic_2015 { () => ({ $ crate :: rt :: begin_panic ("explicit panic") }) , ($ msg : expr $ (,) ?) => ({ $ crate :: rt :: begin_panic ($ msg) ; }) , ("{}" , $ arg : expr $ (,) ?) => ({ $ crate :: rt :: panic_display (&$ arg) ; }) , ($ fmt : expr , $ ($ arg : tt) +) => ({ $ crate :: rt :: panic_fmt ($ crate :: const_format_args ! ($ fmt , $ ($ arg) +)) ; }) , }
};
}
