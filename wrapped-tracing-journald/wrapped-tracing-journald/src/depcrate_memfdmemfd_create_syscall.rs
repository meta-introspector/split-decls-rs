// Generated macro for memfd_create_syscall (function)
macro_rules! Depcrate_memfdmemfd_create_syscall {
() => {
// Module: crate::memfd
// Provides: {"memfd_create_syscall"}
// Dependencies: {}
# [doc = " Make the `memfd_create` syscall ourself instead of going through `libc`;"] # [doc = " `memfd_create` isn't supported on `glibc<2.27` so this allows us to"] # [doc = " support old-but-still-used distros like Ubuntu Xenial, Debian Stretch,"] # [doc = " RHEL 7, etc."] # [doc = ""] # [doc = " See: https://github.com/tokio-rs/tracing/issues/1879"] fn memfd_create_syscall (flags : c_uint) -> c_int { unsafe { syscall (SYS_memfd_create , "tracing-journald\0" . as_ptr () as * const c_char , flags ,) as c_int } }
};
}
