// Generated macro for impl_1310 (impl)
macro_rules! Depcrate_mock_commandimpl_1310 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1310"}
// Dependencies: {}
# [doc = " To simplify life for using a `CommandCreator` across multiple threads."] impl < T : CommandCreator + 'static + Send > CommandCreatorSync for Arc < Mutex < T > > { type Cmd = T :: Cmd ; fn new (client : & Client) -> Arc < Mutex < T > > { Arc :: new (Mutex :: new (T :: new (client))) } fn new_command_sync < S : AsRef < OsStr > > (& mut self , program : S) -> T :: Cmd { self . lock () . unwrap () . new_command (program) } }
};
}
