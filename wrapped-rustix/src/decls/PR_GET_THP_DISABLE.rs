macro_rules! PR_GET_THP_DISABLE {
    () => {
        const PR_GET_THP_DISABLE : c_int = 42 ;
    };
}

PR_GET_THP_DISABLE!();