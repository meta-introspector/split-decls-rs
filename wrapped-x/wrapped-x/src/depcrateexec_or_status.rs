// Generated macro for exec_or_status (function)
macro_rules! Depcrateexec_or_status {
() => {
// Module: crate
// Provides: {"exec_or_status"}
// Dependencies: {}
# [cfg (not (unix))] fn exec_or_status (command : & mut Command) -> io :: Result < ExitStatus > { command . status () }
};
}
