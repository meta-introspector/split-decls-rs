macro_rules! deps {
    () => {
        FileLock!();
        Guard!();
    };
}

macro_rules! Lock {
    () => {
        deps!();
        pub (crate) struct Lock { intraprocess_guard : Guard , lockfile : FileLock , }
    };
}

Lock!()