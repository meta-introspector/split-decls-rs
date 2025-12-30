// Generated macro for impl_155 (impl)
macro_rules! Depcrate_extra_checksimpl_155 {
() => {
// Module: crate::extra_checks
// Provides: {"impl_155"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: MissingReq (a , b , ex) => { write ! (f , "{a} is required to run {b} but it could not be located. Is it installed?") ? ; if let Some (s) = ex { write ! (f , "\n{s}") ? ; } ; Ok (()) } Self :: Version { program , required , installed } => write ! (f , "insufficient version of '{program}' to run external tools: \
                {required} required but found {installed}" ,) , Self :: Generic (s) => f . write_str (s) , Self :: Io (e) => write ! (f , "IO error: {e}") , Self :: FailedCheck (s) => write ! (f , "checks with external tool '{s}' failed") , } } }
};
}
