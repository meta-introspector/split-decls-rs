// Generated macro for DiskStat (struct)
macro_rules! Depcrate_unix_linux_diskDiskStat {
() => {
// Module: crate::unix::linux::disk
// Provides: {"DiskStat"}
// Dependencies: {}
# [doc = " Disk IO stat information from `/proc/diskstats` file."] # [doc = ""] # [doc = " To fully understand these fields, please see the"] # [doc = " [iostats.txt](https://www.kernel.org/doc/Documentation/iostats.txt) kernel documentation."] # [doc = ""] # [doc = " This type only contains the value `sysinfo` is interested into."] # [doc = ""] # [doc = " The fields of this file are:"] # [doc = " 1. major number"] # [doc = " 2. minor number"] # [doc = " 3. device name"] # [doc = " 4. reads completed successfully"] # [doc = " 5. reads merged"] # [doc = " 6. sectors read"] # [doc = " 7. time spent reading (ms)"] # [doc = " 8. writes completed"] # [doc = " 9. writes merged"] # [doc = " 10. sectors written"] # [doc = " 11. time spent writing (ms)"] # [doc = " 12. I/Os currently in progress"] # [doc = " 13. time spent doing I/Os (ms)"] # [doc = " 14. weighted time spent doing I/Os (ms)"] # [doc = ""] # [doc = " Doc reference: https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats"] # [doc = ""] # [doc = " Doc reference: https://www.kernel.org/doc/Documentation/iostats.txt"] # [derive (Debug , PartialEq)] struct DiskStat { sectors_read : u64 , sectors_written : u64 , }
};
}
