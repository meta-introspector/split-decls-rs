macro_rules! deps {
    () => {
        LockState!();
    };
}

macro_rules! LockData {
    () => {
        deps!();
        struct LockData { mutex : Mutex < LockState > , serial : ReentrantMutex < () > , condvar : Condvar , }
    };
}

LockData!();