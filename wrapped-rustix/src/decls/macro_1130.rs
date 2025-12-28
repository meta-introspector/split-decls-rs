macro_rules! macro_1130 {
    () => {
        bitflags :: bitflags ! { # [doc = " `O_*` flags for use with [`openpt`] and [`ioctl_tiocgptpeer`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct OpenptFlags : u32 { # [doc = " `O_RDWR`"] const RDWR = c :: O_RDWR as c :: c_uint ; # [doc = " `O_NOCTTY`"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "l4re" , target_os = "redox" , target_os = "vita")))] const NOCTTY = c :: O_NOCTTY as c :: c_uint ; # [doc = " `O_CLOEXEC`"] # [doc = ""] # [doc = " The standard `posix_openpt` function doesn't support `CLOEXEC`, but"] # [doc = " rustix supports it on Linux, and FreeBSD and NetBSD support it."] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "netbsd"))] const CLOEXEC = c :: O_CLOEXEC as c :: c_uint ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1130!();