macro_rules! PR_GET_TSC {
    () => {
        const PR_GET_TSC : c_int = 25 ;
    };
}

PR_GET_TSC!()