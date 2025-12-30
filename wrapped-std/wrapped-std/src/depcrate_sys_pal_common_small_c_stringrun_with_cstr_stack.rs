// Generated macro for run_with_cstr_stack (function)
macro_rules! Depcrate_sys_pal_common_small_c_stringrun_with_cstr_stack {
() => {
// Module: crate::sys::pal::common::small_c_string
// Provides: {"run_with_cstr_stack"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " `bytes` must have a length less than `MAX_STACK_ALLOCATION`."] unsafe fn run_with_cstr_stack < T > (bytes : & [u8] , f : & dyn Fn (& CStr) -> io :: Result < T > ,) -> io :: Result < T > { let mut buf = MaybeUninit :: < [u8 ; MAX_STACK_ALLOCATION] > :: uninit () ; let buf_ptr = buf . as_mut_ptr () as * mut u8 ; unsafe { ptr :: copy_nonoverlapping (bytes . as_ptr () , buf_ptr , bytes . len ()) ; buf_ptr . add (bytes . len ()) . write (0) ; } match CStr :: from_bytes_with_nul (unsafe { slice :: from_raw_parts (buf_ptr , bytes . len () + 1) }) { Ok (s) => f (s) , Err (_) => Err (NUL_ERR) , } }
};
}
