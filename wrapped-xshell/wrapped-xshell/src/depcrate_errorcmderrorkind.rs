// Generated macro for CmdErrorKind (enum)
macro_rules! Depcrate_errorCmdErrorKind {
() => {
// Module: crate::error
// Provides: {"CmdErrorKind"}
// Dependencies: {}
pub (crate) enum CmdErrorKind { Io (io :: Error) , Utf8 (FromUtf8Error) , Status (ExitStatus) , Timeout , }
};
}
