macro_rules! Guard {
    () => {
        enum Guard { NotLocked , Locked (# [allow (dead_code)] MutexGuard < 'static , () >) , }
    };
}

Guard!()