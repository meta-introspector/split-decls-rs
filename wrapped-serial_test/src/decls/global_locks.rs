macro_rules! deps {
    () => {
        UniqueReentrantMutex!();
    };
}

macro_rules! global_locks {
    () => {
        deps!();
        # [inline] pub (crate) fn global_locks () -> & 'static HashMap < String , UniqueReentrantMutex > { # [cfg (feature = "test_logging")] let _ = env_logger :: builder () . try_init () ; static LOCKS : OnceCell < HashMap < String , UniqueReentrantMutex > > = OnceCell :: new () ; LOCKS . get_or_init (HashMap :: new) }
    };
}

global_locks!()