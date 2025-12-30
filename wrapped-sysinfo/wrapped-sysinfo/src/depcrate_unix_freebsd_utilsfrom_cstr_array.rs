// Generated macro for from_cstr_array (function)
macro_rules! Depcrate_unix_freebsd_utilsfrom_cstr_array {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"from_cstr_array"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) unsafe fn from_cstr_array (ptr : * const * const libc :: c_char) -> Vec < OsString > { if ptr . is_null () { return Vec :: new () ; } let mut max = 0 ; loop { unsafe { let ptr = ptr . add (max) ; if (* ptr) . is_null () { break ; } } max += 1 ; } if max == 0 { return Vec :: new () ; } let mut ret = Vec :: with_capacity (max) ; for pos in 0 .. max { unsafe { let p = ptr . add (pos) ; ret . push (OsStr :: from_bytes (CStr :: from_ptr (* p) . to_bytes ()) . to_os_string ()) ; } } ret }
};
}
