// Generated macro for set_virtual_memory_region_name (function)
macro_rules! Depcrate_process_prctlset_virtual_memory_region_name {
() => {
// Module: crate::process::prctl
// Provides: {"set_virtual_memory_region_name"}
// Dependencies: {}
# [doc = " Set the name for a virtual memory region."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_VMA,PR_SET_VMA_ANON_NAME,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_VMA,PR_SET_VMA_ANON_NAME,…)`]: https://lwn.net/Articles/867818/"] # [inline] # [doc (alias = "PR_SET_VMA")] # [doc (alias = "PR_SET_VMA_ANON_NAME")] pub fn set_virtual_memory_region_name (region : & [u8] , name : Option < & CStr >) -> io :: Result < () > { unsafe { syscalls :: prctl (PR_SET_VMA , PR_SET_VMA_ANON_NAME as * mut _ , region . as_ptr () as * mut _ , region . len () as * mut _ , name . map_or_else (null , CStr :: as_ptr) as * mut _ ,) . map (| _r | ()) } }
};
}
