// Generated macro for ChildOrCall (enum)
macro_rules! Depcrate_mock_commandChildOrCall {
() => {
// Module: crate::mock_command
// Provides: {"ChildOrCall"}
// Dependencies: {}
pub enum ChildOrCall { Child (Result < MockChild >) , Call (Box < dyn Fn (& [OsString]) -> Result < MockChild > + Send >) , }
};
}
