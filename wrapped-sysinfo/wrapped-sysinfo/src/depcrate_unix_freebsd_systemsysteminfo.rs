// Generated macro for SystemInfo (struct)
macro_rules! Depcrate_unix_freebsd_systemSystemInfo {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"SystemInfo"}
// Dependencies: {}
# [doc = " This struct is used to get system information more easily."] # [derive (Debug)] struct SystemInfo { hw_physical_memory : [c_int ; 2] , page_size : c_int , virtual_page_count : [c_int ; 4] , virtual_wire_count : [c_int ; 4] , virtual_active_count : [c_int ; 4] , virtual_cache_count : [c_int ; 4] , virtual_inactive_count : [c_int ; 4] , virtual_free_count : [c_int ; 4] , buf_space : [c_int ; 2] , kd : NonNull < libc :: kvm_t > , # [doc = " From FreeBSD manual: \"The kernel fixed-point scale factor\". It's used when computing"] # [doc = " processes' CPU usage."] fscale : f32 , procstat : * mut libc :: procstat , zfs : Zfs , }
};
}
