// Generated macro for fetch (function)
macro_rules! Depcrate_weakfetch {
() => {
// Module: crate::weak
// Provides: {"fetch"}
// Dependencies: {}
unsafe fn fetch (name : & str) -> * mut c_void { let name = match CStr :: from_bytes_with_nul (name . as_bytes ()) { Ok (c_str) => c_str , Err (..) => return null_mut () , } ; libc :: dlsym (libc :: RTLD_DEFAULT , name . as_ptr () . cast ()) }
};
}
