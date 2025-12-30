// Generated macro for impl_406 (impl)
macro_rules! Depcrate_revisionimpl_406 {
() => {
// Module: crate::revision
// Provides: {"impl_406"}
// Dependencies: {}
impl From < Revision > for OptionalAtomicRevision { fn from (value : Revision) -> Self { Self { data : AtomicUsize :: new (value . as_usize ()) , } } }
};
}
