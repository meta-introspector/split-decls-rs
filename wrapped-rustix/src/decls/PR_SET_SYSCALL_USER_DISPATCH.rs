macro_rules! PR_SET_SYSCALL_USER_DISPATCH {
    () => {
        const PR_SET_SYSCALL_USER_DISPATCH : c_int = 59 ;
    };
}

PR_SET_SYSCALL_USER_DISPATCH!();