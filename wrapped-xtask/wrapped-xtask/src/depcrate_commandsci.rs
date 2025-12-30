// Generated macro for ci (function)
macro_rules! Depcrate_commandsci {
() => {
// Module: crate::commands
// Provides: {"ci"}
// Dependencies: {}
# [doc = " Run CI checks (lint, build, test)"] fn ci () -> Result < () > { lint () ? ; build () ? ; test () ? ; Ok (()) }
};
}
