// Generated macro for test_line_buffer (function)
macro_rules! Depcrate_io_buffered_teststest_line_buffer {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_line_buffer"}
// Dependencies: {}
# [test] fn test_line_buffer () { let mut writer = LineWriter :: new (Vec :: new ()) ; writer . write (& [0]) . unwrap () ; assert_eq ! (* writer . get_ref () , []) ; writer . write (& [1]) . unwrap () ; assert_eq ! (* writer . get_ref () , []) ; writer . flush () . unwrap () ; assert_eq ! (* writer . get_ref () , [0 , 1]) ; writer . write (& [0 , b'\n' , 1 , b'\n' , 2]) . unwrap () ; assert_eq ! (* writer . get_ref () , [0 , 1 , 0 , b'\n' , 1 , b'\n']) ; writer . flush () . unwrap () ; assert_eq ! (* writer . get_ref () , [0 , 1 , 0 , b'\n' , 1 , b'\n' , 2]) ; writer . write (& [3 , b'\n']) . unwrap () ; assert_eq ! (* writer . get_ref () , [0 , 1 , 0 , b'\n' , 1 , b'\n' , 2 , 3 , b'\n']) ; }
};
}
