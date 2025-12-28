macro_rules! deps {
    () => {
        Sleep!();
        StartHandler!();
        PanicHandler!();
        ReleaseThreadHandler!();
        ThreadInfo!();
        AcquireThreadHandler!();
        DeadlockHandler!();
        ExitHandler!();
        JobRef!();
    };
}

macro_rules! Registry {
    () => {
        deps!();
        pub struct Registry { thread_infos : Vec < ThreadInfo > , sleep : Sleep , injected_jobs : Injector < JobRef > , broadcasts : Mutex < Vec < Worker < JobRef > > > , panic_handler : Option < Box < PanicHandler > > , pub (crate) deadlock_handler : Option < Box < DeadlockHandler > > , start_handler : Option < Box < StartHandler > > , exit_handler : Option < Box < ExitHandler > > , pub (crate) acquire_thread_handler : Option < Box < AcquireThreadHandler > > , pub (crate) release_thread_handler : Option < Box < ReleaseThreadHandler > > , terminate_count : AtomicUsize , }
    };
}

Registry!()