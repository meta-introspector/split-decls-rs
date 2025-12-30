// Generated macro for run_with_cstr_allocating (function)
macro_rules! Depcrate_sys_pal_common_small_c_stringrun_with_cstr_allocating {
() => {
// Module: crate::sys::pal::common::small_c_string
// Provides: {"run_with_cstr_allocating"}
// Dependencies: {}
# [cold] # [inline (never)] fn run_with_cstr_allocating < T > (bytes : & [u8] , f : & dyn Fn (& CStr) -> io :: Result < T >) -> io :: Result < T > { match CString :: new (bytes) { Ok (s) => f (& s) , Err (_) => Err (NUL_ERR) , } }
};
}
