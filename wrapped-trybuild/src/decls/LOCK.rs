macro_rules! LOCK {
    () => {
        static LOCK : Mutex < () > = Mutex :: new (()) ;
    };
}

LOCK!();