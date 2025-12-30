// Generated macro for print (macro)
macro_rules! Depcrate_termprint {
() => {
// Module: crate::term
// Provides: {"print"}
// Dependencies: {}
# [deny (unused_macros)] macro_rules ! print { ($ ($ args : tt) *) => { { use std :: io :: Write ; let _ = std :: write ! ($ crate :: term :: lock () , $ ($ args) *) ; } } ; }
};
}
