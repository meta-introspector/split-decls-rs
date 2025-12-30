// Generated macro for tidy_error_ext (macro)
macro_rules! Depcratetidy_error_ext {
() => {
// Module: crate
// Provides: {"tidy_error_ext"}
// Dependencies: {}
macro_rules ! tidy_error_ext { ($ tidy_error : path , $ bad : expr , $ ($ fmt : tt) *) => ({ $ tidy_error (& format_args ! ($ ($ fmt) *) . to_string ()) . expect ("failed to output error") ; *$ bad = true ; }) ; }
};
}
