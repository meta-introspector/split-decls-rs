// Generated macro for is_generic (function)
macro_rules! Depcrateis_generic {
() => {
// Module: crate
// Provides: {"is_generic"}
// Dependencies: {}
fn is_generic < 'tcx > (instance : Instance < 'tcx >) -> bool { instance . args . non_erasable_generics () . next () . is_some () }
};
}
