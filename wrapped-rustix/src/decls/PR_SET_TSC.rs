macro_rules! PR_SET_TSC {
    () => {
        const PR_SET_TSC : c_int = 26 ;
    };
}

PR_SET_TSC!();