// Generated macro for impl_21 (impl)
macro_rules! Depcrate_pathimpl_21 {
() => {
// Module: crate::path
// Provides: {"impl_21"}
// Dependencies: {}
impl CanonicalPath { pub (crate) fn new (path : & Path) -> Self { if let Ok (canonical) = path . canonicalize () { CanonicalPath (canonical) } else { CanonicalPath (path . to_owned ()) } } }
};
}
