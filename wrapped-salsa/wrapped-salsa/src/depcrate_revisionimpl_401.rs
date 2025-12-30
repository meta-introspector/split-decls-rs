// Generated macro for impl_401 (impl)
macro_rules! Depcrate_revisionimpl_401 {
() => {
// Module: crate::revision
// Provides: {"impl_401"}
// Dependencies: {}
impl From < Revision > for AtomicRevision { fn from (value : Revision) -> Self { Self { data : AtomicUsize :: new (value . as_usize ()) , } } }
};
}
