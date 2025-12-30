// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_mock_commandimpl_1293 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1293"}
// Dependencies: {}
# [doc = " Trivial implementation of `CommandCreator` for `ProcessCommandCreator`."] impl CommandCreator for ProcessCommandCreator { type Cmd = AsyncCommand ; fn new (client : & Client) -> ProcessCommandCreator { ProcessCommandCreator { jobserver : client . clone () , } } fn new_command < S : AsRef < OsStr > > (& mut self , program : S) -> AsyncCommand { AsyncCommand :: new (program , self . jobserver . clone ()) } }
};
}
