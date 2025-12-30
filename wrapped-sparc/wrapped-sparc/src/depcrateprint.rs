// Generated macro for print (macro)
macro_rules! Depcrateprint {
() => {
// Module: crate
// Provides: {"print"}
// Dependencies: {}
macro_rules ! print { ($ ($ tt : tt) *) => { { use core :: fmt :: Write as _ ; let _ = write ! (sim :: Console , $ ($ tt) *) ; } } ; }
};
}
