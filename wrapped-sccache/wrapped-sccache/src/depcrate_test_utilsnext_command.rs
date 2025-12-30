// Generated macro for next_command (function)
macro_rules! Depcrate_test_utilsnext_command {
() => {
// Module: crate::test::utils
// Provides: {"next_command"}
// Dependencies: {}
pub fn next_command (creator : & Arc < Mutex < MockCommandCreator > > , child : Result < MockChild >) { creator . lock () . unwrap () . next_command_spawns (child) ; }
};
}
