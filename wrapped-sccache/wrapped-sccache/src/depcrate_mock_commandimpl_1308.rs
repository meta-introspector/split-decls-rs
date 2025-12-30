// Generated macro for impl_1308 (impl)
macro_rules! Depcrate_mock_commandimpl_1308 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1308"}
// Dependencies: {}
impl MockCommandCreator { # [doc = " The next `MockCommand` created will return `child` from `RunCommand::spawn`."] # [allow (dead_code)] pub fn next_command_spawns (& mut self , child : Result < MockChild >) { self . children . push (ChildOrCall :: Child (child)) ; } # [doc = " The next `MockCommand` created will call `call` with the command-line"] # [doc = " arguments passed to the command."] # [allow (dead_code)] pub fn next_command_calls < C > (& mut self , call : C) where C : Fn (& [OsString]) -> Result < MockChild > + Send + 'static , { self . children . push (ChildOrCall :: Call (Box :: new (call))) ; } }
};
}
