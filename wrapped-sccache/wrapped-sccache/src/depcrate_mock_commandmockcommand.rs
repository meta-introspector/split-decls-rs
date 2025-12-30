// Generated macro for MockCommand (struct)
macro_rules! Depcrate_mock_commandMockCommand {
() => {
// Module: crate::mock_command
// Provides: {"MockCommand"}
// Dependencies: {}
# [doc = " A mocked command that simply returns its `child` from `spawn`."] # [allow (dead_code)] # [derive (Debug)] pub struct MockCommand { pub child : Option < ChildOrCall > , pub args : Vec < OsString > , }
};
}
