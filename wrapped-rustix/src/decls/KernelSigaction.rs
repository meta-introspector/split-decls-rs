macro_rules! deps {
    () => {
        KernelSigSet!();
        KernelSighandler!();
        KernelSigrestore!();
    };
}

macro_rules! KernelSigaction {
    () => {
        deps!();
        # [doc = " `kernel_sigaction`"] # [doc = ""] # [doc = " On some architectures, the `sa_restorer` field is omitted."] # [doc = ""] # [doc = " This type does not have the same layout as `libc::sigaction`."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] # [repr (C)] pub struct KernelSigaction { pub sa_handler_kernel : KernelSighandler , pub sa_flags : KernelSigactionFlags , # [cfg (not (any (target_arch = "csky" , target_arch = "loongarch64" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "riscv32" , target_arch = "riscv64")))] pub sa_restorer : KernelSigrestore , pub sa_mask : KernelSigSet , }
    };
}

KernelSigaction!();