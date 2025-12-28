macro_rules! GLOBAL_CLIENT_CHECKED {
    () => {
        static GLOBAL_CLIENT_CHECKED : OnceLock < Client > = OnceLock :: new () ;
    };
}

GLOBAL_CLIENT_CHECKED!();