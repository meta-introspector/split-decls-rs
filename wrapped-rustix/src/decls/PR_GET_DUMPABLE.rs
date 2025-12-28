macro_rules! PR_GET_DUMPABLE {
    () => {
        const PR_GET_DUMPABLE : c_int = 3 ;
    };
}

PR_GET_DUMPABLE!();