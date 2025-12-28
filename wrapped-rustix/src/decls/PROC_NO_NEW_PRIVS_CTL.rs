macro_rules! PROC_NO_NEW_PRIVS_CTL {
    () => {
        const PROC_NO_NEW_PRIVS_CTL : c_int = 19 ;
    };
}

PROC_NO_NEW_PRIVS_CTL!();