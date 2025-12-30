// Generated macro for transmute_data_ptr (function)
macro_rules! Depcrate_zalsatransmute_data_ptr {
() => {
// Module: crate::zalsa
// Provides: {"transmute_data_ptr"}
// Dependencies: {}
# [doc = " Given a wide pointer `T`, extracts the data pointer (typed as `U`)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `U` must be correct type for the data pointer."] pub unsafe fn transmute_data_ptr < T : ? Sized , U > (t : & T) -> & U { let t : * const T = t ; let u : * const U = t as * const U ; unsafe { & * u } }
};
}
