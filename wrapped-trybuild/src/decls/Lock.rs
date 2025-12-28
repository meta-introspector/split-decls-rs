macro_rules! deps {
    () => {
        Guard!();
        FileLock!();
    };
}

macro_rules! Lock {
    () => {
        deps!();
        pub (crate) struct Lock { intraprocess_guard : Guard , lockfile : FileLock , }
    };
}

Lock!();