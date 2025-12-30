// Generated macro for get_actual_device_name (function)
macro_rules! Depcrate_unix_linux_diskget_actual_device_name {
() => {
// Module: crate::unix::linux::disk
// Provides: {"get_actual_device_name"}
// Dependencies: {}
# [doc = " Resolves the actual device name for a specified `device` from `/proc/mounts`"] # [doc = ""] # [doc = " This function is inspired by the [`bottom`] crate implementation and essentially does the following:"] # [doc = "     1. Canonicalizes the specified device path to its absolute form"] # [doc = "     2. Strips the \"/dev\" prefix from the canonicalized path"] # [doc = ""] # [doc = " [`bottom`]: <https://github.com/ClementTsang/bottom/blob/main/src/data_collection/disks/unix/linux/partition.rs#L44>"] fn get_actual_device_name (device : & OsStr) -> String { let device_path = PathBuf :: from (device) ; std :: fs :: canonicalize (& device_path) . ok () . and_then (| path | path . strip_prefix ("/dev") . ok () . map (Path :: to_path_buf)) . unwrap_or (device_path) . to_str () . map (str :: to_owned) . unwrap_or_default () }
};
}
