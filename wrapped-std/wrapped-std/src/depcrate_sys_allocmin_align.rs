// Generated macro for MIN_ALIGN (const)
macro_rules! Depcrate_sys_allocMIN_ALIGN {
() => {
// Module: crate::sys::alloc
// Provides: {"MIN_ALIGN"}
// Dependencies: {}
# [allow (dead_code)] const MIN_ALIGN : usize = if cfg ! (any (all (target_arch = "riscv32" , any (target_os = "espidf" , target_os = "zkvm")) , all (target_arch = "xtensa" , target_os = "espidf") ,)) { 4 } else if cfg ! (any (target_arch = "x86" , target_arch = "arm" , target_arch = "m68k" , target_arch = "csky" , target_arch = "loongarch32" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "sparc" , target_arch = "wasm32" , target_arch = "hexagon" , target_arch = "riscv32" , target_arch = "xtensa" ,)) { 8 } else if cfg ! (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "loongarch64" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "s390x" , target_arch = "sparc64" , target_arch = "riscv64" , target_arch = "wasm64" ,)) { 16 } else { panic ! ("add a value for MIN_ALIGN") } ;
};
}
