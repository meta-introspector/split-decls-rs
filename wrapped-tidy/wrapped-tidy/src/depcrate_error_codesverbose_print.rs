// Generated macro for verbose_print (macro)
macro_rules! Depcrate_error_codesverbose_print {
() => {
// Module: crate::error_codes
// Provides: {"verbose_print"}
// Dependencies: {}
macro_rules ! verbose_print { ($ verbose : expr , $ ($ fmt : tt) *) => { if $ verbose { println ! ("{}" , format_args ! ($ ($ fmt) *)) ; } } ; }
};
}
