// Generated macro for transmute_data_mut_ptr (function)
macro_rules! Depcrate_zalsatransmute_data_mut_ptr {
() => {
// Module: crate::zalsa
// Provides: {"transmute_data_mut_ptr"}
// Dependencies: {}
# [doc = " Given a wide pointer `T`, extracts the data pointer (typed as `U`)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `U` must be correct type for the data pointer."] pub (crate) unsafe fn transmute_data_mut_ptr < T : ? Sized , U > (t : & mut T) -> & mut U { let t : * mut T = t ; let u : * mut U = t as * mut U ; unsafe { & mut * u } }
};
}
