// Generated macro for mlock_with (function)
macro_rules! Depcrate_mm_mmapmlock_with {
() => {
// Module: crate::mm::mmap
// Provides: {"mlock_with"}
// Dependencies: {}
# [doc = " `mlock2(ptr, len, flags)`—Lock memory into RAM, with flags."] # [doc = ""] # [doc = " `mlock_with` is the same as [`mlock`] but adds an additional flags operand."] # [doc = ""] # [doc = " Some implementations implicitly round the memory region out to the nearest"] # [doc = " page boundaries, so this function may lock more memory than explicitly"] # [doc = " requested if the memory isn't page-aligned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The range of memory starting at `ptr`, rounded down to the applicable page"] # [doc = " boundary, and extending for `len` bytes, rounded up to the applicable page"] # [doc = " size, must be valid to read with `ptr`'s provenance."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [glibc]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mlock2.2.html"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Page-Lock-Functions.html#index-mlock2"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "mlock2")] pub unsafe fn mlock_with (ptr : * mut c_void , len : usize , flags : MlockFlags) -> io :: Result < () > { backend :: mm :: syscalls :: mlock_with (ptr , len , flags) }
};
}
