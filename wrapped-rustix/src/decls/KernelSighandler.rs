macro_rules! KernelSighandler {
    () => {
        # [doc = " `__kernel_sighandler_t`"] # [doc = ""] # [doc = " This type differs from `libc::sighandler_t`, but can be transmuted to it."] pub type KernelSighandler = Option < unsafe extern "C" fn (arg1 : crate :: ffi :: c_int) > ;
    };
}

KernelSighandler!();