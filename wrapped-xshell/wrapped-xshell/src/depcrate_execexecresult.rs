// Generated macro for ExecResult (struct)
macro_rules! Depcrate_execExecResult {
() => {
// Module: crate::exec
// Provides: {"ExecResult"}
// Dependencies: {}
# [derive (Default , Debug)] pub (crate) struct ExecResult { pub (crate) stdout : Vec < u8 > , pub (crate) stderr : Vec < u8 > , pub (crate) status : Option < ExitStatus > , pub (crate) error : Option < io :: Error > , }
};
}
