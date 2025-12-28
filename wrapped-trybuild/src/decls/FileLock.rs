macro_rules! FileLock {
    () => {
        enum FileLock { NotLocked , Locked { path : PathBuf , done : Arc < AtomicBool > , } , }
    };
}

FileLock!()