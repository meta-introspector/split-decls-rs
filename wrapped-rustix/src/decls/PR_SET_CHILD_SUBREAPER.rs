macro_rules! PR_SET_CHILD_SUBREAPER {
    () => {
        const PR_SET_CHILD_SUBREAPER : c_int = 36 ;
    };
}

PR_SET_CHILD_SUBREAPER!();