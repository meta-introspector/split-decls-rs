// Generated macro for impl_121 (impl)
macro_rules! Depcrate_common_diskimpl_121 {
() => {
// Module: crate::common::disk
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Disks { type Item = & 'a Disk ; type IntoIter = std :: slice :: Iter < 'a , Disk > ; fn into_iter (self) -> Self :: IntoIter { self . list () . iter () } }
};
}
