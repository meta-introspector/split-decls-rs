// Generated macro for impl_1302 (impl)
macro_rules! Depcrate_mock_commandimpl_1302 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1302"}
// Dependencies: {}
# [async_trait] impl CommandChild for MockChild { type I = io :: Cursor < Vec < u8 > > ; type O = io :: Cursor < Vec < u8 > > ; type E = io :: Cursor < Vec < u8 > > ; fn take_stdin (& mut self) -> Option < io :: Cursor < Vec < u8 > > > { self . stdin . take () } fn take_stdout (& mut self) -> Option < io :: Cursor < Vec < u8 > > > { self . stdout . take () } fn take_stderr (& mut self) -> Option < io :: Cursor < Vec < u8 > > > { self . stderr . take () } async fn wait (mut self) -> io :: Result < ExitStatus > { self . wait_result . take () . unwrap () } async fn wait_with_output (self) -> io :: Result < Output > { let MockChild { stdout , stderr , wait_result , .. } = self ; wait_result . unwrap () . map (| status | Output { status , stdout : stdout . map (| c | c . into_inner ()) . unwrap_or_else (Vec :: new) , stderr : stderr . map (| c | c . into_inner ()) . unwrap_or_else (Vec :: new) , }) } }
};
}
