// Generated macro for CommandChild (trait)
macro_rules! Depcrate_mock_commandCommandChild {
() => {
// Module: crate::mock_command
// Provides: {"CommandChild"}
// Dependencies: {}
# [doc = " A trait that provides a subset of the methods of `std::process::Child`."] # [async_trait] pub trait CommandChild { # [doc = " The type of the process' standard input."] type I : AsyncWrite + Unpin + Sync + Send + 'static ; # [doc = " The type of the process' standard output."] type O : AsyncRead + Unpin + Sync + Send + 'static ; # [doc = " The type of the process' standard error."] type E : AsyncRead + Unpin + Sync + Send + 'static ; # [doc = " Take the stdin object from the process, if available."] fn take_stdin (& mut self) -> Option < Self :: I > ; # [doc = " Take the stdout object from the process, if available."] fn take_stdout (& mut self) -> Option < Self :: O > ; # [doc = " Take the stderr object from the process, if available."] fn take_stderr (& mut self) -> Option < Self :: E > ; # [doc = " Wait for the process to complete and return its exit status."] async fn wait (self) -> io :: Result < ExitStatus > ; # [doc = " Wait for the process to complete and return its output."] async fn wait_with_output (self) -> io :: Result < Output > ; }
};
}
