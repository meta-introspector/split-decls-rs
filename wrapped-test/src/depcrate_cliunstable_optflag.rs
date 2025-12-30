// Generated macro for unstable_optflag (macro)
macro_rules! Depcrate_cliunstable_optflag {
() => {
// Module: crate::cli
// Provides: {"unstable_optflag"}
// Dependencies: {}
macro_rules ! unstable_optflag { ($ matches : ident , $ allow_unstable : ident , $ option_name : literal) => { { let opt = $ matches . opt_present ($ option_name) ; if !$ allow_unstable && opt { return Err (format ! ("The \"{}\" flag is only accepted on the nightly compiler with -Z unstable-options" , $ option_name)) ; } opt } } ; }
};
}
