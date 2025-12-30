// Generated macro for get_disk_io (function)
macro_rules! Depcrate_unix_apple_macos_diskget_disk_io {
() => {
// Module: crate::unix::apple::macos::disk
// Provides: {"get_disk_io"}
// Dependencies: {}
# [doc = " Returns a tuple consisting of the total number of bytes read and written by the specified disk"] pub (crate) fn get_disk_io (bsd_name : & [u8]) -> Option < (u64 , u64) > { let stat_string = CFString :: from_static_str (ffi :: kIOBlockStorageDriverStatisticsKey) ; iterate_service_tree (bsd_name , & stat_string , | parent_entry , properties | { if ! unsafe { IOObjectConformsTo (parent_entry , c"IOBlockStorageDriver" . as_ptr () as * mut _) } { return None ; } unsafe { let read_bytes = super :: disk :: get_int_value (properties , Some (& CFString :: from_static_str (ffi :: kIOBlockStorageDriverStatisticsBytesReadKey ,)) ,) ? ; let written_bytes = super :: disk :: get_int_value (properties , Some (& CFString :: from_static_str (ffi :: kIOBlockStorageDriverStatisticsBytesWrittenKey ,)) ,) ? ; Some ((read_bytes , written_bytes)) } }) }
};
}
