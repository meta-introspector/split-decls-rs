// Generated macro for impl_1309 (impl)
macro_rules! Depcrate_mock_commandimpl_1309 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1309"}
// Dependencies: {}
impl CommandCreator for MockCommandCreator { type Cmd = MockCommand ; fn new (_client : & Client) -> MockCommandCreator { MockCommandCreator { children : Vec :: new () , } } fn new_command < S : AsRef < OsStr > > (& mut self , _program : S) -> MockCommand { assert ! (! self . children . is_empty () , "Too many calls to MockCommandCreator::new_command, or not enough to MockCommandCreator::new_command_spawns!") ; MockCommand { child : Some (self . children . remove (0)) , args : vec ! [] , } } }
};
}
