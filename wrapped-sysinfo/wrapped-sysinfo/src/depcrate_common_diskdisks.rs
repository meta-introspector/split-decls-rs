// Generated macro for Disks (struct)
macro_rules! Depcrate_common_diskDisks {
() => {
// Module: crate::common::disk
// Provides: {"Disks"}
// Dependencies: {}
# [doc = " Disks interface."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Disks;"] # [doc = ""] # [doc = " let disks = Disks::new_with_refreshed_list();"] # [doc = " for disk in disks.list() {"] # [doc = "     println!(\"{disk:?}\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ⚠\u{fe0f} Note that tmpfs mounts are excluded by default under Linux."] # [doc = " To display tmpfs mount points, the `linux-tmpfs` feature must be enabled."] # [doc = ""] # [doc = " ⚠\u{fe0f} Note that network devices are excluded by default under Linux."] # [doc = " To display mount points using the CIFS and NFS protocols, the `linux-netdevs`"] # [doc = " feature must be enabled. Note, however, that sysinfo may hang under certain"] # [doc = " circumstances. For example, if a CIFS or NFS share has been mounted with"] # [doc = " the _hard_ option, but the connection has an error, such as the share server has stopped."] pub struct Disks { inner : crate :: DisksInner , }
};
}
