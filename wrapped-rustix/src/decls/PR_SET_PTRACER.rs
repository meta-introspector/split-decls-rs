macro_rules! PR_SET_PTRACER {
    () => {
        const PR_SET_PTRACER : c_int = 0x59_61_6d_61 ;
    };
}

PR_SET_PTRACER!();