// Generated macro for test_buffered_writer (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_writer {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_writer"}
// Dependencies: {}
# [test] fn test_buffered_writer () { let inner = Vec :: new () ; let mut writer = BufWriter :: with_capacity (2 , inner) ; writer . write (& [0 , 1]) . unwrap () ; assert_eq ! (writer . buffer () , []) ; assert_eq ! (* writer . get_ref () , [0 , 1]) ; writer . write (& [2]) . unwrap () ; assert_eq ! (writer . buffer () , [2]) ; assert_eq ! (* writer . get_ref () , [0 , 1]) ; writer . write (& [3]) . unwrap () ; assert_eq ! (writer . buffer () , [2 , 3]) ; assert_eq ! (* writer . get_ref () , [0 , 1]) ; writer . flush () . unwrap () ; assert_eq ! (writer . buffer () , []) ; assert_eq ! (* writer . get_ref () , [0 , 1 , 2 , 3]) ; writer . write (& [4]) . unwrap () ; writer . write (& [5]) . unwrap () ; assert_eq ! (writer . buffer () , [4 , 5]) ; assert_eq ! (* writer . get_ref () , [0 , 1 , 2 , 3]) ; writer . write (& [6]) . unwrap () ; assert_eq ! (writer . buffer () , [6]) ; assert_eq ! (* writer . get_ref () , [0 , 1 , 2 , 3 , 4 , 5]) ; writer . write (& [7 , 8]) . unwrap () ; assert_eq ! (writer . buffer () , []) ; assert_eq ! (* writer . get_ref () , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8]) ; writer . write (& [9 , 10 , 11]) . unwrap () ; assert_eq ! (writer . buffer () , []) ; assert_eq ! (* writer . get_ref () , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11]) ; writer . flush () . unwrap () ; assert_eq ! (writer . buffer () , []) ; assert_eq ! (* writer . get_ref () , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11]) ; }
};
}
