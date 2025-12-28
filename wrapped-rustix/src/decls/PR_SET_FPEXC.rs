macro_rules! PR_SET_FPEXC {
    () => {
        const PR_SET_FPEXC : c_int = 12 ;
    };
}

PR_SET_FPEXC!();