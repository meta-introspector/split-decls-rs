macro_rules! deps {
    () => {
        RestoreEnv!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a > RestoreEnv < 'a > { # [doc = " Capture the given variables from the environment."] # [doc = ""] # [doc = " `guard` holds a lock on the shared mutex for exclusive access to the environment, to make"] # [doc = " sure that the environment gets restored while the lock is still held, i.e the current"] # [doc = " thread still has exclusive access to the environment."] fn capture < I > (guard : ReentrantMutexGuard < 'a , () > , vars : I) -> Self where I : Iterator < Item = & 'a OsStr > + 'a , { let env = vars . map (| v | (v , env :: var_os (v))) . collect () ; Self { env , _guard : guard } } }
    };
}

impl_4!();