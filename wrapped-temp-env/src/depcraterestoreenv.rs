// Generated macro for RestoreEnv (struct)
macro_rules! DepcrateRestoreEnv {
() => {
// Module: crate
// Provides: {"RestoreEnv"}
// Dependencies: {}
struct RestoreEnv < 'a > { env : HashMap < & 'a OsStr , Option < OsString > > , _guard : ReentrantMutexGuard < 'a , () > , }
};
}
