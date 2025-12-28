macro_rules! SYSCALL_DISPATCH_FILTER_ALLOW {
    () => {
        # [doc = " Allow system calls to be executed."] const SYSCALL_DISPATCH_FILTER_ALLOW : u8 = 0 ;
    };
}

SYSCALL_DISPATCH_FILTER_ALLOW!()