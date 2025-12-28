macro_rules! deps {
    () => {
        Result!();
        FileLock!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl FileLock { fn acquire (path : impl AsRef < Path >) -> Result < Self > { let path = path . as_ref () . to_owned () ; let Some (lockfile) = create (& path) else { return Ok (FileLock :: NotLocked) ; } ; let done = Arc :: new (AtomicBool :: new (false)) ; let thread = thread :: Builder :: new () . name ("trybuild-flock" . to_owned ()) ; thread . spawn ({ let done = Arc :: clone (& done) ; move | | poll (lockfile , done) }) ? ; Ok (FileLock :: Locked { path , done }) } }
    };
}

impl_98!()