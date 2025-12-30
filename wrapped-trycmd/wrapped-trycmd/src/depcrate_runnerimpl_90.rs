// Generated macro for impl_90 (impl)
macro_rules! Depcrate_runnerimpl_90 {
() => {
// Module: crate::runner
// Provides: {"impl_90"}
// Dependencies: {}
impl Filesystem { fn is_ok (& self) -> bool { if self . context . is_empty () { true } else { self . context . iter () . all (FileStatus :: is_ok) } } }
};
}
