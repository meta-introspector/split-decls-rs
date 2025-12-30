// Generated macro for Command (struct)
macro_rules! Depcrate_cmdCommand {
() => {
// Module: crate::cmd
// Provides: {"Command"}
// Dependencies: {}
# [doc = " Process spawning for testing of non-interactive commands"] # [derive (Debug)] pub struct Command { cmd : std :: process :: Command , stdin : Option < crate :: Data > , timeout : Option < std :: time :: Duration > , _stderr_to_stdout : bool , config : crate :: Assert , }
};
}
