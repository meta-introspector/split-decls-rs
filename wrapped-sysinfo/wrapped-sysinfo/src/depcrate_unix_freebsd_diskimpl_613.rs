// Generated macro for impl_613 (impl)
macro_rules! Depcrate_unix_freebsd_diskimpl_613 {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"impl_613"}
// Dependencies: {}
impl crate :: DisksInner { pub (crate) fn new () -> Self { Self { disks : Vec :: with_capacity (2) , } } pub (crate) fn refresh_specifics (& mut self , remove_not_listed_disks : bool , refresh_kind : DiskRefreshKind ,) { unsafe { get_all_list (& mut self . disks , remove_not_listed_disks , refresh_kind) } } pub (crate) fn list (& self) -> & [Disk] { & self . disks } pub (crate) fn list_mut (& mut self) -> & mut [Disk] { & mut self . disks } }
};
}
