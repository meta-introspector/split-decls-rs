// Generated macro for suppressible_tidy_err (macro)
macro_rules! Depcrate_stylesuppressible_tidy_err {
() => {
// Module: crate::style
// Provides: {"suppressible_tidy_err"}
// Dependencies: {}
macro_rules ! suppressible_tidy_err { ($ err : ident , $ skip : ident , $ msg : literal) => { if let Directive :: Deny = $ skip { $ err (& format ! ($ msg)) ; } else { $ skip = Directive :: Ignore (true) ; } } ; }
};
}
