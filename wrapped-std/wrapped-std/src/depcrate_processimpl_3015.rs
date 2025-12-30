// Generated macro for impl_3015 (impl)
macro_rules! Depcrate_processimpl_3015 {
() => {
// Module: crate::process
// Provides: {"impl_3015"}
// Dependencies: {}
# [unstable (feature = "exit_status_error" , issue = "84908")] impl fmt :: Display for ExitStatusError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "process exited unsuccessfully: {}" , self . into_status ()) } }
};
}
