macro_rules! PR_SET_KEEPCAPS {
    () => {
        const PR_SET_KEEPCAPS : c_int = 8 ;
    };
}

PR_SET_KEEPCAPS!();