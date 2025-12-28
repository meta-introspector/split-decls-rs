macro_rules! PR_GET_SECCOMP {
    () => {
        const PR_GET_SECCOMP : c_int = 21 ;
    };
}

PR_GET_SECCOMP!()