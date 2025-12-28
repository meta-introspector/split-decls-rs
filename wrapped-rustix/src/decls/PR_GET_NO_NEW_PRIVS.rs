macro_rules! PR_GET_NO_NEW_PRIVS {
    () => {
        const PR_GET_NO_NEW_PRIVS : c_int = 39 ;
    };
}

PR_GET_NO_NEW_PRIVS!()