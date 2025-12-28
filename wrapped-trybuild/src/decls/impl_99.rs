macro_rules! deps {
    () => {
        Lock!();
        Guard!();
        FileLock!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Drop for Lock { fn drop (& mut self) { let Lock { intraprocess_guard , lockfile , } = self ; * lockfile = FileLock :: NotLocked ; * intraprocess_guard = Guard :: NotLocked ; } }
    };
}

impl_99!()