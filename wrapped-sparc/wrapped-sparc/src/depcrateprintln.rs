// Generated macro for println (macro)
macro_rules! Depcrateprintln {
() => {
// Module: crate
// Provides: {"println"}
// Dependencies: {}
macro_rules ! println { ($ ($ tt : tt) *) => { { use core :: fmt :: Write as _ ; let _ = writeln ! (sim :: Console , $ ($ tt) *) ; } } ; }
};
}
