// Generated macro for impl_122 (impl)
macro_rules! Depcrate_common_diskimpl_122 {
() => {
// Module: crate::common::disk
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Disks { type Item = & 'a mut Disk ; type IntoIter = std :: slice :: IterMut < 'a , Disk > ; fn into_iter (self) -> Self :: IntoIter { self . list_mut () . iter_mut () } }
};
}
