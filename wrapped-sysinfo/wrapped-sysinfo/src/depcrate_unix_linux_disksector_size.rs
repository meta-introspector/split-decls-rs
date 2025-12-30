// Generated macro for SECTOR_SIZE (const)
macro_rules! Depcrate_unix_linux_diskSECTOR_SIZE {
() => {
// Module: crate::unix::linux::disk
// Provides: {"SECTOR_SIZE"}
// Dependencies: {}
# [doc = " Copied from [`psutil`]:"] # [doc = ""] # [doc = " \"man iostat\" states that sectors are equivalent with blocks and have"] # [doc = " a size of 512 bytes. Despite this value can be queried at runtime"] # [doc = " via /sys/block/{DISK}/queue/hw_sector_size and results may vary"] # [doc = " between 1k, 2k, or 4k... 512 appears to be a magic constant used"] # [doc = " throughout Linux source code:"] # [doc = " * <https://stackoverflow.com/a/38136179/376587>"] # [doc = " * <https://lists.gt.net/linux/kernel/2241060>"] # [doc = " * <https://github.com/giampaolo/psutil/issues/1305>"] # [doc = " * <https://github.com/torvalds/linux/blob/4f671fe2f9523a1ea206f63fe60a7c7b3a56d5c7/include/linux/bio.h#L99>"] # [doc = " * <https://lkml.org/lkml/2015/8/17/234>"] # [doc = ""] # [doc = " [`psutil`]: <https://github.com/giampaolo/psutil/blob/master/psutil/_pslinux.py#L103>"] const SECTOR_SIZE : u64 = 512 ;
};
}
