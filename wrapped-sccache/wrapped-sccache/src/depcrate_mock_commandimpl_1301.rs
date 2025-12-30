// Generated macro for impl_1301 (impl)
macro_rules! Depcrate_mock_commandimpl_1301 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1301"}
// Dependencies: {}
# [doc = " A mocked child process that simply returns stored values for its status and output."] impl MockChild { # [doc = " Create a `MockChild` that will return the specified `status`, `stdout`, and `stderr` when waited upon."] # [allow (dead_code)] pub fn new < T : AsRef < [u8] > , U : AsRef < [u8] > > (status : ExitStatus , stdout : T , stderr : U ,) -> MockChild { MockChild { stdin : Some (io :: Cursor :: new (vec ! [])) , stdout : Some (io :: Cursor :: new (stdout . as_ref () . to_vec ())) , stderr : Some (io :: Cursor :: new (stderr . as_ref () . to_vec ())) , wait_result : Some (Ok (status)) , } } # [doc = " Create a `MockChild` that will return the specified `err` when waited upon."] # [allow (dead_code)] pub fn with_error (err : io :: Error) -> MockChild { MockChild { stdin : None , stdout : None , stderr : None , wait_result : Some (Err (err)) , } } }
};
}
