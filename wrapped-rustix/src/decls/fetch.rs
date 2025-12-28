macro_rules! fetch {
    () => {
        unsafe fn fetch (name : & str) -> * mut c_void { let name = match CStr :: from_bytes_with_nul (name . as_bytes ()) { Ok (c_str) => c_str , Err (..) => return null_mut () , } ; libc :: dlsym (libc :: RTLD_DEFAULT , name . as_ptr () . cast ()) }
    };
}

fetch!();