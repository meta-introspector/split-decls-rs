// Generated macro for DiskKind (enum)
macro_rules! Depcrate_common_diskDiskKind {
() => {
// Module: crate::common::disk
// Provides: {"DiskKind"}
// Dependencies: {}
# [doc = " Enum containing the different supported kinds of disks."] # [doc = ""] # [doc = " This type is returned by [`Disk::kind`](`crate::Disk::kind`)."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Disks;"] # [doc = ""] # [doc = " let disks = Disks::new_with_refreshed_list();"] # [doc = " for disk in disks.list() {"] # [doc = "     println!(\"{:?}: {:?}\", disk.name(), disk.kind());"] # [doc = " }"] # [doc = " ```"] # [derive (Debug , Clone , Copy , Hash , PartialEq , Eq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub enum DiskKind { # [doc = " HDD type."] HDD , # [doc = " SSD type."] SSD , # [doc = " Unknown type."] Unknown (isize) , }
};
}
