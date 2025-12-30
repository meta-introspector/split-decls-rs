// Generated macro for impl_85 (impl)
macro_rules! Depcrate_consoleimpl_85 {
() => {
// Module: crate::console
// Provides: {"impl_85"}
// Dependencies: {}
impl ConsoleTestDiscoveryState { pub (crate) fn new (opts : & TestOpts) -> io :: Result < ConsoleTestDiscoveryState > { let log_out = match opts . logfile { Some (ref path) => Some (File :: create (path) ?) , None => None , } ; Ok (ConsoleTestDiscoveryState { log_out , tests : 0 , benchmarks : 0 , ignored : 0 }) } pub (crate) fn write_log < F , S > (& mut self , msg : F) -> io :: Result < () > where S : AsRef < str > , F : FnOnce () -> S , { match self . log_out { None => Ok (()) , Some (ref mut o) => { let msg = msg () ; let msg = msg . as_ref () ; o . write_all (msg . as_bytes ()) } } } }
};
}
