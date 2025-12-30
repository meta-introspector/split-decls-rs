// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_mock_commandimpl_1287 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1287"}
// Dependencies: {}
# [doc = " Trivial implementation of `CommandChild` for `std::process::Child`."] # [async_trait] impl CommandChild for Child { type I = ChildStdin ; type O = ChildStdout ; type E = ChildStderr ; fn take_stdin (& mut self) -> Option < ChildStdin > { self . inner . stdin . take () } fn take_stdout (& mut self) -> Option < ChildStdout > { self . inner . stdout . take () } fn take_stderr (& mut self) -> Option < ChildStderr > { self . inner . stderr . take () } async fn wait (self) -> io :: Result < ExitStatus > { let Child { mut inner , token } = self ; inner . wait () . await . inspect (| _ret | { drop (token) ; }) } async fn wait_with_output (self) -> io :: Result < Output > { let Child { inner , token } = self ; inner . wait_with_output () . await . inspect (| _ret | { drop (token) ; }) } }
};
}
