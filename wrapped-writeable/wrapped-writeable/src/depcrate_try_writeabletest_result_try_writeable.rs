// Generated macro for test_result_try_writeable (function)
macro_rules! Depcrate_try_writeabletest_result_try_writeable {
() => {
// Module: crate::try_writeable
// Provides: {"test_result_try_writeable"}
// Dependencies: {}
# [test] fn test_result_try_writeable () { let mut result : Result < & str , usize > = Ok ("success") ; assert_try_writeable_eq ! (result , "success") ; result = Err (44) ; assert_try_writeable_eq ! (result , "44" , Err (44)) ; assert_try_writeable_parts_eq ! (result , "44" , Err (44) , [(0 , 2 , Part :: ERROR)]) }
};
}
