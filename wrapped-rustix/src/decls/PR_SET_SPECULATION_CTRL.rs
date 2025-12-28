macro_rules! PR_SET_SPECULATION_CTRL {
    () => {
        const PR_SET_SPECULATION_CTRL : c_int = 53 ;
    };
}

PR_SET_SPECULATION_CTRL!()