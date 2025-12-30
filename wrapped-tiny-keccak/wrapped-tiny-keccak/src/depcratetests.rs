// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { left_encode , right_encode } ; # [test] fn test_left_encode () { assert_eq ! (left_encode (0) . value () , & [1 , 0]) ; assert_eq ! (left_encode (128) . value () , & [1 , 128]) ; assert_eq ! (left_encode (65536) . value () , & [3 , 1 , 0 , 0]) ; assert_eq ! (left_encode (4096) . value () , & [2 , 16 , 0]) ; assert_eq ! (left_encode (54321) . value () , & [2 , 212 , 49]) ; } # [test] fn test_right_encode () { assert_eq ! (right_encode (0) . value () , & [0 , 1]) ; assert_eq ! (right_encode (128) . value () , & [128 , 1]) ; assert_eq ! (right_encode (65536) . value () , & [1 , 0 , 0 , 3]) ; assert_eq ! (right_encode (4096) . value () , & [16 , 0 , 2]) ; assert_eq ! (right_encode (54321) . value () , & [212 , 49 , 2]) ; } }
};
}
