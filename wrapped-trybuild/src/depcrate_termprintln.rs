// Generated macro for println (macro)
macro_rules! Depcrate_termprintln {
() => {
// Module: crate::term
// Provides: {"println"}
// Dependencies: {}
# [deny (unused_macros)] macro_rules ! println { ($ ($ args : tt) *) => { { use std :: io :: Write ; let _ = std :: writeln ! ($ crate :: term :: lock () , $ ($ args) *) ; } } ; }
};
}
