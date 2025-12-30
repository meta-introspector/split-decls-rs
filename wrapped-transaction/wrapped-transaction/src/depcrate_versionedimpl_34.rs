// Generated macro for impl_34 (impl)
macro_rules! Depcrate_versionedimpl_34 {
() => {
// Module: crate::versioned
// Provides: {"impl_34"}
// Dependencies: {}
impl From < Transaction > for VersionedTransaction { fn from (transaction : Transaction) -> Self { Self { signatures : transaction . signatures , message : VersionedMessage :: Legacy (transaction . message) , } } }
};
}
