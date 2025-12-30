// Generated macro for vprintln (macro)
macro_rules! Depcratevprintln {
() => {
// Module: crate
// Provides: {"vprintln"}
// Dependencies: {}
macro_rules ! vprintln { ($ ($ x : expr) ,*) => { # [cfg (feature = "std")] if crate :: VERBOSE { std :: println ! ($ ($ x) ,*) ; } } }
};
}
