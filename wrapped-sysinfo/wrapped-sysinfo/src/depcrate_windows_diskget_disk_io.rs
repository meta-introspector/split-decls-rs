// Generated macro for get_disk_io (function)
macro_rules! Depcrate_windows_diskget_disk_io {
() => {
// Module: crate::windows::disk
// Provides: {"get_disk_io"}
// Dependencies: {}
# [doc = " Returns a tuple consisting of the total number of bytes read and written by the volume with the"] # [doc = " specified device path"] fn get_disk_io (handle : HandleWrapper) -> Option < (u64 , u64) > { let mut disk_perf = DISK_PERFORMANCE :: default () ; let mut bytes_returned = 0 ; unsafe { DeviceIoControl (handle . 0 , IOCTL_DISK_PERFORMANCE , None , 0 , Some (& mut disk_perf as * mut _ as _) , size_of :: < DISK_PERFORMANCE > () as u32 , Some (& mut bytes_returned) , None ,) } . inspect_err (| _err | { sysinfo_debug ! ("Error: DeviceIoControl(IOCTL_DISK_PERFORMANCE) = {:?}" , _err) ; }) . ok () ? ; Some ((disk_perf . BytesRead . try_into () . ok () ? , disk_perf . BytesWritten . try_into () . ok () ? ,)) }
};
}
