macro_rules! PR_SET_THP_DISABLE {
    () => {
        const PR_SET_THP_DISABLE : c_int = 41 ;
    };
}

PR_SET_THP_DISABLE!()