// Generated macro for get_vendor_id_and_brand (function)
macro_rules! Depcrate_windows_cpuget_vendor_id_and_brand {
() => {
// Module: crate::windows::cpu
// Provides: {"get_vendor_id_and_brand"}
// Dependencies: {}
# [cfg (all (not (target_arch = "x86_64") , not (target_arch = "x86")))] pub (crate) fn get_vendor_id_and_brand (info : & SYSTEM_INFO) -> (String , String) { (get_vendor_id_not_great (info) , String :: new ()) }
};
}
