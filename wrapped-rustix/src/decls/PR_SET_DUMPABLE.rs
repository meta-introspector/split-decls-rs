macro_rules! PR_SET_DUMPABLE {
    () => {
        const PR_SET_DUMPABLE : c_int = 4 ;
    };
}

PR_SET_DUMPABLE!();