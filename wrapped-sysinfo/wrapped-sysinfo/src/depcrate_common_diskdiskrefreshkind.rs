// Generated macro for DiskRefreshKind (struct)
macro_rules! Depcrate_common_diskDiskRefreshKind {
() => {
// Module: crate::common::disk
// Provides: {"DiskRefreshKind"}
// Dependencies: {}
# [doc = " Used to determine what you want to refresh specifically on the [`Disk`] type."] # [doc = ""] # [doc = " * `kind` is about refreshing the [`Disk::kind`] information."] # [doc = " * `storage` is about refreshing the [`Disk::available_space`] and [`Disk::total_space`] information."] # [doc = " * `io_usage` is about refreshing the [`Disk::usage`] information."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{Disks, DiskRefreshKind};"] # [doc = ""] # [doc = " let mut disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::everything());"] # [doc = ""] # [doc = " for disk in disks.list() {"] # [doc = "     assert!(disk.total_space() != 0);"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Default)] pub struct DiskRefreshKind { kind : bool , storage : bool , io_usage : bool , }
};
}
