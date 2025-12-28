macro_rules! PR_SET_TIMERSLACK {
    () => {
        const PR_SET_TIMERSLACK : c_int = 29 ;
    };
}

PR_SET_TIMERSLACK!();