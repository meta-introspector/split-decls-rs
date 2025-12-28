macro_rules! PR_SET_SECCOMP {
    () => {
        const PR_SET_SECCOMP : c_int = 22 ;
    };
}

PR_SET_SECCOMP!()