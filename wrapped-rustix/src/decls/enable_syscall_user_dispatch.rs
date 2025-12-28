macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! enable_syscall_user_dispatch {
    () => {
        deps!();
        # [doc = " Enable Syscall User Dispatch mechanism."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SYSCALL_USER_DISPATCH,PR_SYS_DISPATCH_ON,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_SYSCALL_USER_DISPATCH,PR_SYS_DISPATCH_ON,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub unsafe fn enable_syscall_user_dispatch (always_allowed_region : & [u8] , fast_switch_flag : & AtomicU8 ,) -> io :: Result < () > { syscalls :: prctl (PR_SET_SYSCALL_USER_DISPATCH , PR_SYS_DISPATCH_ON as * mut _ , always_allowed_region . as_ptr () as * mut _ , always_allowed_region . len () as * mut _ , as_ptr (fast_switch_flag) as * mut _ ,) . map (| _r | ()) }
    };
}

enable_syscall_user_dispatch!();