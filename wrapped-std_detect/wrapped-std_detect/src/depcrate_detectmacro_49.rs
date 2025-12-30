// Generated macro for macro_49 (macro)
macro_rules! Depcrate_detectmacro_49 {
() => {
// Module: crate::detect
// Provides: {"macro_49"}
// Dependencies: {}
cfg_select ! { miri => { # [path = "os/other.rs"] mod os ; } any (target_arch = "x86" , target_arch = "x86_64") => { # [path = "os/x86.rs"] mod os ; } all (any (target_os = "linux" , target_os = "android") , feature = "libc") => { # [cfg (any (target_arch = "riscv32" , target_arch = "riscv64"))] # [path = "os/riscv.rs"] mod riscv ; # [path = "os/linux/mod.rs"] mod os ; } all (target_os = "freebsd" , feature = "libc") => { # [cfg (target_arch = "aarch64")] # [path = "os/aarch64.rs"] mod aarch64 ; # [path = "os/freebsd/mod.rs"] mod os ; } all (target_os = "openbsd" , target_arch = "aarch64" , feature = "libc") => { # [allow (dead_code)] # [path = "os/aarch64.rs"] mod aarch64 ; # [path = "os/openbsd/aarch64.rs"] mod os ; } all (target_os = "windows" , any (target_arch = "aarch64" , target_arch = "arm64ec")) => { # [path = "os/windows/aarch64.rs"] mod os ; } all (target_vendor = "apple" , target_arch = "aarch64" , feature = "libc") => { # [path = "os/darwin/aarch64.rs"] mod os ; } _ => { # [path = "os/other.rs"] mod os ; } }
};
}
