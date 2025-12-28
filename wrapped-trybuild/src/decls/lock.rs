macro_rules! deps {
    () => {
        Term!();
    };
}

macro_rules! lock {
    () => {
        deps!();
        pub (crate) fn lock () -> MutexGuard < 'static , Term > { TERM . get_or_init (| | Mutex :: new (Term :: new ())) . lock () . unwrap_or_else (PoisonError :: into_inner) }
    };
}

lock!()