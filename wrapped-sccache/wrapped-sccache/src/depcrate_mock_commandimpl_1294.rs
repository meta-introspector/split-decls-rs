// Generated macro for impl_1294 (impl)
macro_rules! Depcrate_mock_commandimpl_1294 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1294"}
// Dependencies: {}
# [doc = " Trivial implementation of `CommandCreatorSync` for `ProcessCommandCreator`."] impl CommandCreatorSync for ProcessCommandCreator { type Cmd = AsyncCommand ; fn new (client : & Client) -> ProcessCommandCreator { CommandCreator :: new (client) } fn new_command_sync < S : AsRef < OsStr > > (& mut self , program : S) -> AsyncCommand { self . new_command (program) } }
};
}
