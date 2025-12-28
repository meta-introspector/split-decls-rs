macro_rules! deps {
    () => {
        Term!();
    };
}

macro_rules! TERM {
    () => {
        deps!();
        static TERM : OnceLock < Mutex < Term > > = OnceLock :: new () ;
    };
}

TERM!()