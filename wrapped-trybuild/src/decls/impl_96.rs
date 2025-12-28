macro_rules! deps {
    () => {
        FileLock!();
        Guard!();
        Lock!();
        Result!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Lock { pub fn acquire (path : impl AsRef < Path >) -> Result < Self > { Ok (Lock { intraprocess_guard : Guard :: acquire () , lockfile : FileLock :: acquire (path) ? , }) } }
    };
}

impl_96!()