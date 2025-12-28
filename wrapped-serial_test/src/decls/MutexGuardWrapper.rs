macro_rules! deps {
    () => {
        Locks!();
    };
}

macro_rules! MutexGuardWrapper {
    () => {
        deps!();
        pub (crate) struct MutexGuardWrapper < 'a > { # [allow (dead_code)] mutex_guard : ReentrantMutexGuard < 'a , () > , locks : Locks , }
    };
}

MutexGuardWrapper!();