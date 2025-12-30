// Generated macro for CommandCreatorSync (trait)
macro_rules! Depcrate_mock_commandCommandCreatorSync {
() => {
// Module: crate::mock_command
// Provides: {"CommandCreatorSync"}
// Dependencies: {}
# [doc = " A trait for simplifying the normal case while still allowing the mock case requiring mutability."] pub trait CommandCreatorSync : Clone + Send + Sync + 'static { type Cmd : RunCommand ; fn new (client : & Client) -> Self ; fn new_command_sync < S : AsRef < OsStr > > (& mut self , program : S) -> Self :: Cmd ; }
};
}
