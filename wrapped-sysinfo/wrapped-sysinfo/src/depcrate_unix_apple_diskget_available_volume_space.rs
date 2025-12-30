// Generated macro for get_available_volume_space (function)
macro_rules! Depcrate_unix_apple_diskget_available_volume_space {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_available_volume_space"}
// Dependencies: {}
fn get_available_volume_space (disk_props : & CFDictionary) -> Option < u64 > { unsafe { get_int_value (disk_props , kCFURLVolumeAvailableCapacityForImportantUsageKey ,) . filter (| bytes | * bytes != 0) . or_else (| | get_int_value (disk_props , kCFURLVolumeAvailableCapacityKey)) } }
};
}
