// Generated macro for get_cmdline_from_buffer (function)
macro_rules! Depcrate_windows_processget_cmdline_from_buffer {
() => {
// Module: crate::windows::process
// Provides: {"get_cmdline_from_buffer"}
// Dependencies: {}
unsafe fn get_cmdline_from_buffer (buffer : PCWSTR) -> Vec < OsString > { let mut argc = MaybeUninit :: < i32 > :: uninit () ; unsafe { let argv_p = CommandLineToArgvW (buffer , argc . as_mut_ptr ()) ; if argv_p . is_null () { return Vec :: new () ; } let argc = argc . assume_init () ; let argv = std :: slice :: from_raw_parts (argv_p , argc as usize) ; let mut res = Vec :: new () ; for arg in argv { res . push (OsString :: from_wide (arg . as_wide ())) ; } let _err = LocalFree (Some (HLOCAL (argv_p as _))) ; res } }
};
}
