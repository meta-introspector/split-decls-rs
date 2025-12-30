// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < T > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < T > { if self . start == self . vec . len () { None } else { self . vec . pop () } } }
};
}
