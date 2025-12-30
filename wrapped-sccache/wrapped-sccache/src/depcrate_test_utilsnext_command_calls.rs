// Generated macro for next_command_calls (function)
macro_rules! Depcrate_test_utilsnext_command_calls {
() => {
// Module: crate::test::utils
// Provides: {"next_command_calls"}
// Dependencies: {}
pub fn next_command_calls < C : Fn (& [OsString]) -> Result < MockChild > + Send + 'static > (creator : & Arc < Mutex < MockCommandCreator > > , call : C ,) { creator . lock () . unwrap () . next_command_calls (call) ; }
};
}
