// Generated macro for run_path_with_cstr (function)
macro_rules! Depcrate_sys_pal_common_small_c_stringrun_path_with_cstr {
() => {
// Module: crate::sys::pal::common::small_c_string
// Provides: {"run_path_with_cstr"}
// Dependencies: {}
# [inline] pub fn run_path_with_cstr < T > (path : & Path , f : & dyn Fn (& CStr) -> io :: Result < T >) -> io :: Result < T > { run_with_cstr (path . as_os_str () . as_encoded_bytes () , f) }
};
}
