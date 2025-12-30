// Generated macro for unstable_optopt (macro)
macro_rules! Depcrate_cliunstable_optopt {
() => {
// Module: crate::cli
// Provides: {"unstable_optopt"}
// Dependencies: {}
macro_rules ! unstable_optopt { ($ matches : ident , $ allow_unstable : ident , $ option_name : literal) => { { let opt = $ matches . opt_str ($ option_name) ; if !$ allow_unstable && opt . is_some () { return Err (format ! ("The \"{}\" option is only accepted on the nightly compiler with -Z unstable-options" , $ option_name)) ; } opt } } ; }
};
}
