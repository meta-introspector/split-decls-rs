// Generated macro for elaborate (function)
macro_rules! Depcrate_elaborateelaborate {
() => {
// Module: crate::elaborate
// Provides: {"elaborate"}
// Dependencies: {}
pub fn elaborate < I : Interner , O : Elaboratable < I > > (cx : I , obligations : impl IntoIterator < Item = O > ,) -> Elaborator < I , O > { let mut elaborator = Elaborator { cx , stack : Vec :: new () , visited : HashSet :: default () , mode : Filter :: All , elaborate_sized : ElaborateSized :: No , } ; elaborator . extend_deduped (obligations) ; elaborator }
};
}
