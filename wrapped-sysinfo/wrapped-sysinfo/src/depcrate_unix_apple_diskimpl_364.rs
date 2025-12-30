// Generated macro for impl_364 (impl)
macro_rules! Depcrate_unix_apple_diskimpl_364 {
() => {
// Module: crate::unix::apple::disk
// Provides: {"impl_364"}
// Dependencies: {}
impl crate :: DisksInner { pub (crate) fn new () -> Self { Self { disks : Vec :: with_capacity (2) , } } pub (crate) fn refresh_specifics (& mut self , remove_not_listed_disks : bool , refresh_kind : DiskRefreshKind ,) { unsafe { with_autorelease (| | { get_list (& mut self . disks , refresh_kind) ; }) } if remove_not_listed_disks { self . disks . retain_mut (| disk | { if ! disk . inner . updated { return false ; } disk . inner . updated = false ; true }) ; } else { for c in self . disks . iter_mut () { c . inner . updated = false ; } } } pub (crate) fn list (& self) -> & [Disk] { & self . disks } pub (crate) fn list_mut (& mut self) -> & mut [Disk] { & mut self . disks } }
};
}
