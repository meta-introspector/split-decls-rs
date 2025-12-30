// Generated macro for impl_380 (impl)
macro_rules! Depcrate_un_dereferimpl_380 {
() => {
// Module: crate::un_derefer
// Provides: {"impl_380"}
// Dependencies: {}
impl < T : Copy > SlicePlusOne < '_ , T > { # [inline] fn read (& self) -> Option < T > { self . slice . first () . copied () . or (self . last) } # [inline] fn advance (& mut self) { match self . slice { [_ , remainder @ ..] => { self . slice = remainder ; } [] => self . last = None , } } }
};
}
