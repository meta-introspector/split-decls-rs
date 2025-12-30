// Generated macro for impl_1117 (impl)
macro_rules! Depcrate_windows_diskimpl_1117 {
() => {
// Module: crate::windows::disk
// Provides: {"impl_1117"}
// Dependencies: {}
impl DisksInner { pub (crate) fn new () -> Self { Self { disks : Vec :: with_capacity (2) , } } pub (crate) fn from_vec (disks : Vec < Disk >) -> Self { Self { disks } } pub (crate) fn into_vec (self) -> Vec < Disk > { self . disks } pub (crate) fn refresh_specifics (& mut self , remove_not_listed_disks : bool , refreshes : DiskRefreshKind ,) { unsafe { get_list (& mut self . disks , remove_not_listed_disks , refreshes) ; } } pub (crate) fn list (& self) -> & [Disk] { & self . disks } pub (crate) fn list_mut (& mut self) -> & mut [Disk] { & mut self . disks } }
};
}
