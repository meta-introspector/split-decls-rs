// Generated macro for impl_976 (impl)
macro_rules! Depcrate_unknown_diskimpl_976 {
() => {
// Module: crate::unknown::disk
// Provides: {"impl_976"}
// Dependencies: {}
impl DisksInner { pub (crate) fn new () -> Self { Self { disks : Vec :: new () } } pub (crate) fn from_vec (disks : Vec < Disk >) -> Self { Self { disks } } pub (crate) fn into_vec (self) -> Vec < Disk > { self . disks } pub (crate) fn refresh_specifics (& mut self , _remove_not_listed_disks : bool , _refreshes : DiskRefreshKind ,) { } pub (crate) fn list (& self) -> & [Disk] { & self . disks } pub (crate) fn list_mut (& mut self) -> & mut [Disk] { & mut self . disks } }
};
}
