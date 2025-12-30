// Generated macro for get (function)
macro_rules! Depcrate_data_runtimeget {
() => {
// Module: crate::data::runtime
// Provides: {"get"}
// Dependencies: {}
pub (crate) fn get () -> std :: sync :: MutexGuard < 'static , Runtime > { static RT : std :: sync :: Mutex < Runtime > = std :: sync :: Mutex :: new (Runtime :: new ()) ; RT . lock () . unwrap_or_else (| poisoned | poisoned . into_inner ()) }
};
}
