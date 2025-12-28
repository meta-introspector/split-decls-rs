macro_rules! PR_GET_FPEXC {
    () => {
        const PR_GET_FPEXC : c_int = 11 ;
    };
}

PR_GET_FPEXC!();