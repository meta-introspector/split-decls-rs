macro_rules! PR_SET_NO_NEW_PRIVS {
    () => {
        const PR_SET_NO_NEW_PRIVS : c_int = 38 ;
    };
}

PR_SET_NO_NEW_PRIVS!();