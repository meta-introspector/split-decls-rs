// Generated macro for get_vendor_id_and_brand (function)
macro_rules! Depcrate_unix_linux_cpuget_vendor_id_and_brand {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"get_vendor_id_and_brand"}
// Dependencies: {}
# [doc = " Returns the brand/vendor string for the first CPU (which should be the same for all CPUs)."] pub (crate) fn get_vendor_id_and_brand () -> HashMap < usize , (String , String) > { let mut s = String :: new () ; if File :: open ("/proc/cpuinfo") . and_then (| mut f | f . read_to_string (& mut s)) . is_err () { return HashMap :: new () ; } get_vendor_id_and_brand_inner (& s) }
};
}
