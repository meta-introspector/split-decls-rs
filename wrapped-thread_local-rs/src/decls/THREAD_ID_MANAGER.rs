macro_rules! deps {
    () => {
        ThreadIdManager!();
    };
}

macro_rules! THREAD_ID_MANAGER {
    () => {
        deps!();
        static THREAD_ID_MANAGER : Mutex < ThreadIdManager > = Mutex :: new (ThreadIdManager :: new ()) ;
    };
}

THREAD_ID_MANAGER!();