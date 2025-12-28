macro_rules! deps {
    () => {
        KernelSigaction!();
    };
}

macro_rules! macro_1404 {
    () => {
        deps!();
        bitflags :: bitflags ! { # [doc = " Flags for use with [`KernelSigaction`]."] # [doc = ""] # [doc = " This type does not have the same layout as `sa_flags` field in"] # [doc = " `libc::sigaction`, however the flags have the same values as their"] # [doc = " libc counterparts."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug , Default)] pub struct KernelSigactionFlags : crate :: ffi :: c_ulong { # [doc = " `SA_NOCLDSTOP`"] const NOCLDSTOP = linux_raw_sys :: general :: SA_NOCLDSTOP as _ ; # [doc = " `SA_NOCLDWAIT` (since Linux 2.6)"] const NOCLDWAIT = linux_raw_sys :: general :: SA_NOCLDWAIT as _ ; # [doc = " `SA_NODEFER`"] const NODEFER = linux_raw_sys :: general :: SA_NODEFER as _ ; # [doc = " `SA_ONSTACK`"] const ONSTACK = linux_raw_sys :: general :: SA_ONSTACK as _ ; # [doc = " `SA_RESETHAND`"] const RESETHAND = linux_raw_sys :: general :: SA_RESETHAND as _ ; # [doc = " `SA_RESTART`"] const RESTART = linux_raw_sys :: general :: SA_RESTART as _ ; # [doc = " `SA_RESTORER`"] # [cfg (not (any (target_arch = "csky" , target_arch = "loongarch64" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "riscv32" , target_arch = "riscv64")))] const RESTORER = linux_raw_sys :: general :: SA_RESTORER as _ ; # [doc = " `SA_SIGINFO` (since Linux 2.2)"] const SIGINFO = linux_raw_sys :: general :: SA_SIGINFO as _ ; # [doc = " `SA_UNSUPPORTED` (since Linux 5.11)"] const UNSUPPORTED = linux_raw_sys :: general :: SA_UNSUPPORTED as _ ; # [doc = " `SA_EXPOSE_TAGBITS` (since Linux 5.11)"] const EXPOSE_TAGBITS = linux_raw_sys :: general :: SA_EXPOSE_TAGBITS as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1404!();