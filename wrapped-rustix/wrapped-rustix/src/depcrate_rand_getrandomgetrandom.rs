// Generated macro for getrandom (function)
macro_rules! Depcrate_rand_getrandomgetrandom {
() => {
// Module: crate::rand::getrandom
// Provides: {"getrandom"}
// Dependencies: {}
# [doc = " `getrandom(buf, flags)`—Reads a sequence of random bytes."] # [doc = ""] # [doc = " This is a very low-level API which may be difficult to use correctly. Most"] # [doc = " users should prefer to use [`getrandom`] or [`rand`] APIs instead."] # [doc = ""] # [doc = " This function is implemented using a system call, and not the"] # [doc = " [vDSO mechanism] introduced in Linux 6.11. See [#1185] for details."] # [doc = ""] # [doc = " [`getrandom`]: https://crates.io/crates/getrandom"] # [doc = " [`rand`]: https://crates.io/crates/rand"] # [doc = " [vDSO mechanism]: https://lwn.net/Articles/983186/"] # [doc = " [#1185]: https://github.com/bytecodealliance/rustix/issues/1185"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getrandom.2.html"] # [inline] pub fn getrandom < Buf : Buffer < u8 > > (mut buf : Buf , flags : GetRandomFlags) -> io :: Result < Buf :: Output > { let len = unsafe { backend :: rand :: syscalls :: getrandom (buf . parts_mut () , flags) ? } ; unsafe { Ok (buf . assume_init (len)) } }
};
}
