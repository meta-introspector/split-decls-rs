macro_rules! PR_SET_PDEATHSIG {
    () => {
        const PR_SET_PDEATHSIG : c_int = 1 ;
    };
}

PR_SET_PDEATHSIG!()