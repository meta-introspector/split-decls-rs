// Generated macro for style_error (macro)
macro_rules! Depcratestyle_error {
() => {
// Module: crate
// Provides: {"style_error"}
// Dependencies: {}
macro_rules ! style_error { ($ bad : expr , $ path : expr , $ ($ arg : tt) *) => { *$ bad = true ; eprint ! ("error in {}: " , $ path . display ()) ; eprintln ! ("{}" , format_args ! ($ ($ arg) *)) ; } ; }
};
}
