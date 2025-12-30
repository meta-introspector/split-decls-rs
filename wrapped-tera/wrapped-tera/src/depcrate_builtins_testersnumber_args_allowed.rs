// Generated macro for number_args_allowed (function)
macro_rules! Depcrate_builtins_testersnumber_args_allowed {
() => {
// Module: crate::builtins::testers
// Provides: {"number_args_allowed"}
// Dependencies: {}
# [doc = " Check that the number of args match what was expected"] pub fn number_args_allowed (tester_name : & str , max : usize , args_len : usize) -> Result < () > { if max == 0 && args_len > max { return Err (Error :: msg (format ! ("Tester `{}` was called with some args but this test doesn't take args" , tester_name))) ; } if args_len > max { return Err (Error :: msg (format ! ("Tester `{}` was called with {} args, the max number is {}" , tester_name , args_len , max))) ; } Ok (()) }
};
}
