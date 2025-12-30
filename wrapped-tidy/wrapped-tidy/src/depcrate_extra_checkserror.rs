// Generated macro for Error (enum)
macro_rules! Depcrate_extra_checksError {
() => {
// Module: crate::extra_checks
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] enum Error { Io (io :: Error) , # [doc = " a is required to run b. c is extra info"] MissingReq (& 'static str , & 'static str , Option < String >) , # [doc = " Tool x failed the check"] FailedCheck (& 'static str) , # [doc = " Any message, just print it"] Generic (String) , # [doc = " Installed but wrong version"] Version { program : & 'static str , required : & 'static str , installed : String , } , }
};
}
