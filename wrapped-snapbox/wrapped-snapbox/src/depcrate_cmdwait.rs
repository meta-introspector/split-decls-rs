// Generated macro for wait (function)
macro_rules! Depcrate_cmdwait {
() => {
// Module: crate::cmd
// Provides: {"wait"}
// Dependencies: {}
# [cfg (not (feature = "cmd"))] fn wait (mut child : std :: process :: Child , _timeout : Option < std :: time :: Duration > ,) -> std :: io :: Result < std :: process :: ExitStatus > { child . wait () }
};
}
