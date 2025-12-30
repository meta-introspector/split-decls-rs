// Generated macro for memfd_create (function)
macro_rules! Depcrate_fs_memfd_creatememfd_create {
() => {
// Module: crate::fs::memfd_create
// Provides: {"memfd_create"}
// Dependencies: {}
# [doc = " `memfd_create(name, flags)`—Create an anonymous file."] # [doc = ""] # [doc = " For a higher-level API to this functionality, see the [memfd] crate."] # [doc = ""] # [doc = " [memfd]: https://crates.io/crates/memfd"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [glibc]"] # [doc = "  - [FreeBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/memfd_create.2.html"] # [doc = " [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Memory_002dmapped-I_002fO.html#index-memfd_005fcreate"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?memfd_create"] # [inline] pub fn memfd_create < P : path :: Arg > (name : P , flags : MemfdFlags) -> io :: Result < OwnedFd > { name . into_with_c_str (| name | backend :: fs :: syscalls :: memfd_create (name , flags)) }
};
}
