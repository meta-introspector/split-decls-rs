macro_rules! SYSCALL_DISPATCH_FILTER_BLOCK {
    () => {
        # [doc = " Block system calls from executing."] const SYSCALL_DISPATCH_FILTER_BLOCK : u8 = 1 ;
    };
}

SYSCALL_DISPATCH_FILTER_BLOCK!();