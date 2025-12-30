// Generated macro for test (function)
macro_rules! Depcrate_commandstest {
() => {
// Module: crate::commands
// Provides: {"test"}
// Dependencies: {}
# [doc = " Run tests for libs, backends, and docs"] fn test () -> Result < () > { test_libs () ? ; for backend in [Backend :: Crossterm , Backend :: Termion , Backend :: Termwiz] { TestBackend { backend } . run () ? ; } test_docs :: test_docs () ? ; Ok (()) }
};
}
