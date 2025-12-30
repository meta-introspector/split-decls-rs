// Generated macro for impl_43 (impl)
macro_rules! Depcrate_iterator_backendimpl_43 {
() => {
// Module: crate::iterator::backend
// Provides: {"impl_43"}
// Dependencies: {}
impl < E : Exfiltrator > Iterator for Pending < E > { type Item = E :: Output ; fn next (& mut self) -> Option < E :: Output > { while self . position < self . pending . slots . len () { let sig = self . position ; let slot = & self . pending . slots [sig] ; let result = self . pending . exfiltrator . load (slot , sig as c_int) ; if result . is_some () { return result ; } else { self . position += 1 ; } } None } }
};
}
