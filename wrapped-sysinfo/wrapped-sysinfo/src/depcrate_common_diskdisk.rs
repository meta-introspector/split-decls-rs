// Generated macro for Disk (struct)
macro_rules! Depcrate_common_diskDisk {
() => {
// Module: crate::common::disk
// Provides: {"Disk"}
// Dependencies: {}
# [doc = " Struct containing a disk information."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Disks;"] # [doc = ""] # [doc = " let disks = Disks::new_with_refreshed_list();"] # [doc = " for disk in disks.list() {"] # [doc = "     println!(\"{:?}: {:?}\", disk.name(), disk.kind());"] # [doc = " }"] # [doc = " ```"] pub struct Disk { pub (crate) inner : crate :: DiskInner , }
};
}
