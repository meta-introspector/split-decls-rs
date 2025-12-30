// Generated macro for POLYFILL_INTERFACES (static)
macro_rules! Depcrate_constantsPOLYFILL_INTERFACES {
() => {
// Module: crate::constants
// Provides: {"POLYFILL_INTERFACES"}
// Dependencies: {}
pub (crate) static POLYFILL_INTERFACES : Lazy < BTreeSet < & 'static str > > = Lazy :: new (| | BTreeSet :: from_iter (vec ! ["AudioContext" , "OfflineAudioContext"])) ;
};
}
