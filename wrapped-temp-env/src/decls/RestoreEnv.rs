macro_rules! RestoreEnv {
    () => {
        struct RestoreEnv < 'a > { env : HashMap < & 'a OsStr , Option < OsString > > , _guard : ReentrantMutexGuard < 'a , () > , }
    };
}

RestoreEnv!()