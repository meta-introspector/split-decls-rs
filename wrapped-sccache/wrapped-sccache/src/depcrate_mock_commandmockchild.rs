// Generated macro for MockChild (struct)
macro_rules! Depcrate_mock_commandMockChild {
() => {
// Module: crate::mock_command
// Provides: {"MockChild"}
// Dependencies: {}
# [doc = " A struct that mocks `std::process::Child`."] # [allow (dead_code)] # [derive (Debug)] pub struct MockChild { # [doc = " A `Cursor` to hand out as stdin."] pub stdin : Option < io :: Cursor < Vec < u8 > > > , # [doc = " A `Cursor` to hand out as stdout."] pub stdout : Option < io :: Cursor < Vec < u8 > > > , # [doc = " A `Cursor` to hand out as stderr."] pub stderr : Option < io :: Cursor < Vec < u8 > > > , # [doc = " The `Result` to be handed out when `wait` is called."] pub wait_result : Option < io :: Result < ExitStatus > > , }
};
}
