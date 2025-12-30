// Generated macro for run_with_cstr (function)
macro_rules! Depcrate_sys_pal_common_small_c_stringrun_with_cstr {
() => {
// Module: crate::sys::pal::common::small_c_string
// Provides: {"run_with_cstr"}
// Dependencies: {}
# [inline] pub fn run_with_cstr < T > (bytes : & [u8] , f : & dyn Fn (& CStr) -> io :: Result < T >) -> io :: Result < T > { if bytes . len () >= MAX_STACK_ALLOCATION { run_with_cstr_allocating (bytes , f) } else { unsafe { run_with_cstr_stack (bytes , f) } } }
};
}
