// Generated macro for lookup_malloc_address (function)
macro_rules! Depcratelookup_malloc_address {
() => {
// Module: crate
// Provides: {"lookup_malloc_address"}
// Dependencies: {}
fn lookup_malloc_address () -> * const c_char { unsafe { let mut info : libc :: Dl_info = core :: mem :: zeroed () ; let fnptr : unsafe extern "C" fn (libc :: size_t) -> * mut c_void = libc :: malloc ; let fnptr = fnptr as * const c_void ; if libc :: dladdr (fnptr , & mut info) == 0 { libc :: printf (b"failed finding `malloc`\n\0" . as_ptr () . cast ()) ; libc :: abort () ; } info . dli_fname } }
};
}
