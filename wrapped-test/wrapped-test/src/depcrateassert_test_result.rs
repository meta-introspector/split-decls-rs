// Generated macro for assert_test_result (function)
macro_rules! Depcrateassert_test_result {
() => {
// Module: crate
// Provides: {"assert_test_result"}
// Dependencies: {}
# [doc = " Invoked when unit tests terminate. Returns `Result::Err` if the test is"] # [doc = " considered a failure. By default, invokes `report()` and checks for a `0`"] # [doc = " result."] pub fn assert_test_result < T : Termination > (result : T) -> Result < () , String > { let code = result . report () . to_i32 () ; if code == 0 { Ok (()) } else { Err (format ! ("the test returned a termination value with a non-zero status code \
             ({code}) which indicates a failure")) } }
};
}
